/*
Made by:Mathew Dusome
Date: 2025-05-03
Program Details: Central texture manager for preloading and sharing textures

To use this:
1. In your mod.rs file located in the modules folder add the following to the end of the file:
    pub mod preload_image;
    
2. Add the following use command:
    use crate::modules::preload_image::TextureManager;

3. Create and initialize a TextureManager:
    let mut texture_manager = TextureManager::new();
    
4. Preload your textures at startup:
    // Preload a list of textures
    texture_manager.preload_all(&["assets/image1.png", "assets/image2.png"]).await;
    
    // Or preload individual textures
    texture_manager.preload("assets/image3.png").await;
    
5. Get preloaded textures for use with ImageObject - two approaches:

   // Approach 1: Using unwrap() - Simple but will panic if image doesn't exist
   // Only use this when you're certain the texture was preloaded
   image_obj.set_preload(texture_manager.get_preload("assets/image1.png").unwrap());
   
   // Approach 2: Using if let Some() - Safer, handles missing textures gracefully
   if let Some(preloaded) = texture_manager.get_preload("assets/image2.png") {
       img.set_preload(preloaded);
   } else {
       println!("Warning: Image not found in texture manager");
       // Handle the error case (e.g., try to load it or use a placeholder)
   }
    
6. Access textures by index:
    // Using unwrap() approach:
    img.set_preload(texture_manager.get_preload_by_index(0).unwrap());
    
    // Using if let Some() approach:
    if let Some(preloaded) = texture_manager.get_preload_by_index(1) {
        img.set_preload(preloaded);
    }
    
7. Getting the number of preloaded textures:
    let count = texture_manager.texture_count();
    
8. For implementing features like image slideshows, you can increment an index
   and wrap around to cycle through all images:
    current_index = (current_index + 1) % texture_manager.texture_count();
    
    // Using unwrap() (assumes there are textures available):
    img.set_preload(texture_manager.get_preload_by_index(current_index).unwrap());
    
    // Or more safely with error handling:
    if texture_manager.texture_count() > 0 {
        if let Some(preloaded) = texture_manager.get_preload_by_index(current_index) {
            img.set_preload(preloaded);
        }
    }

9. For using with loading screens and coroutines:
    // Create a coroutine for loading textures
    let loading_task = start_coroutine(async move {
        let mut tm = TextureManager::new();
        for (index, asset_path) in assets.iter().enumerate() {
            tm.preload(asset_path).await;
            loading_progress = (index + 1) as f32 / total_assets as f32;
            next_frame().await;
        }
        loading_complete = true;
        tm
    });
    
    // In loading screen loop
    while !loading_complete {
        clear_background(DARKGREEN);
        draw_loading_progress(loading_progress);
        next_frame().await;
    }
    
    // Get the fully loaded texture manager
    let tm = loading_task.await;

Note: For clearing images, use the clear() method directly on the ImageObject:
    img.clear();
*/
use macroquad::texture::Texture2D;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use macroquad::prelude::*;
use macroquad::experimental::coroutines::start_coroutine;
use crate::modules::still_image::set_texture_main;

/// Options for customizing the loading screen appearance
pub struct LoadingScreenOptions {
    /// Title displayed at the top of the loading screen (default: none)
    pub title: Option<String>,
    /// Background color of the loading screen (default: DARKGREEN)
    pub background_color: Color,
    /// Progress bar background color (default: DARKGRAY)
    pub bar_background_color: Color,
    /// Progress bar fill color (default: GREEN)
    pub bar_fill_color: Color,
    /// Text color for all text elements (default: WHITE)
    pub text_color: Color,
    /// File name text color (default: SKYBLUE)
    pub filename_color: Color,
    /// Font size for the title (default: 60)
    pub title_font_size: u16,
    /// Font size for progress text (default: 30)
    pub progress_font_size: u16,
    /// Font size for filename text (default: 20)
    pub filename_font_size: u16,
    /// Whether to show the "Loading Complete!" message (default: true)
    pub show_completion_message: bool,
    /// Custom completion message (default: "Loading Complete!")
    pub completion_message: String,
    /// Delay in seconds after completion before continuing (default: 0.5)
    pub completion_delay: f32,
}

impl Default for LoadingScreenOptions {
    fn default() -> Self {
        Self {
            title: None,
            background_color: DARKGREEN,
            bar_background_color: DARKGRAY,
            bar_fill_color: GREEN,
            text_color: WHITE,
            filename_color: SKYBLUE,
            title_font_size: 60,
            progress_font_size: 30,
            filename_font_size: 20,
            show_completion_message: true,
            completion_message: "Loading Complete!".to_string(),
            completion_delay: 0.5,
        }
    }
}

/// A central texture manager to preload and share textures
/// This reduces memory usage and prevents flickering when switching images
#[derive(Clone)]
pub struct TextureManager {
    textures: Arc<Mutex<HashMap<String, (Texture2D, Option<Vec<u8>>)>>>,
    load_order: Arc<Mutex<Vec<String>>>, // Store just the order textures were loaded in
}

impl TextureManager {
    /// Create a new texture manager
    pub fn new() -> Self {
        Self {
            textures: Arc::new(Mutex::new(HashMap::new())),
            load_order: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Preload a texture by its file path
    pub async fn preload(&self, path: &str) {
        // First, check if the texture already exists
        let texture_exists = {
            let textures = self.textures.lock().unwrap();
            textures.contains_key(path)
        };
        
        // If it doesn't exist, load it
        if !texture_exists {
            // Load the texture outside of any locks
            let (texture, mask) = set_texture_main(path).await;
            
            // Now update the maps with short-lived locks
            {
                let mut textures = self.textures.lock().unwrap();
                textures.insert(path.to_string(), (texture, mask));
            }
            
            {
                let mut load_order = self.load_order.lock().unwrap();
                load_order.push(path.to_string());
            }
        }
    }
    
    /// Preload multiple textures at once
    #[allow(unused)]
    pub async fn preload_all(&self, paths: &[&str]) {
        for path in paths {
            self.preload(path).await;
        }
    }
    
    /// Get a preloaded texture for use in an ImageObject
    #[allow(unused)]
    pub fn get_preload(&self, path: &str) -> Option<(Texture2D, Option<Vec<u8>>, String)> {
        let textures = self.textures.lock().unwrap();
        textures.get(path).map(|(texture, mask)| 
            (texture.clone(), mask.clone(), path.to_string())
        )
    }
    
    /// Get a preloaded texture by its index in the preload order
    #[allow(unused)]
    pub fn get_preload_by_index(&self, index: usize) -> Option<(Texture2D, Option<Vec<u8>>, String)> {
        let load_order = self.load_order.lock().unwrap();
        if index < load_order.len() {
            let path = &load_order[index];
            self.get_preload(path)
        } else {
            None
        }
    }
    
    /// Get the number of preloaded textures
    #[allow(unused)]
    pub fn texture_count(&self) -> usize {
        let load_order = self.load_order.lock().unwrap();
        load_order.len()
    }
    
    /// Get a list of all preloaded texture paths in load order
    #[allow(unused)]
    pub fn get_texture_paths(&self) -> Vec<String> {
        let load_order = self.load_order.lock().unwrap();
        load_order.clone()
    }
    
    /// Load assets with a built-in loading screen that works well for web
    /// This method handles all the complexities of asset loading and progress display
    pub async fn preload_with_loading_screen(&self, assets: Vec<String>, options: Option<LoadingScreenOptions>) {
        // Use default options if none provided
        let options = options.unwrap_or_default();
        
        // Thread-safe progress counters that can be shared between coroutines
        let loaded_counter = Arc::new(AtomicUsize::new(0));
        let total_assets = assets.len();
        
        // Start a background coroutine for loading assets WITHOUT awaiting it
        // This is the key to avoiding black flashes on web
        {
            let assets_to_load = assets.clone();
            let counter = loaded_counter.clone();
            let loading_tm = self.clone(); // Clone the TextureManager for the coroutine
            
            // Important: We start the coroutine but DON'T await it
            start_coroutine(async move {
                for asset_path in assets_to_load {
                    // Load asset into the shared texture manager
                    loading_tm.preload(&asset_path).await;
                    
                    // Update the counter atomically
                    counter.fetch_add(1, Ordering::SeqCst);
                    
                    // Yielding control back to the main thread
                    next_frame().await;
                }
            });
        }
        
        // Main rendering loop for the loading screen
        // This runs in the main thread and never awaits the asset loading
        loop {
            // Read the current progress atomically
            let loaded_assets = loaded_counter.load(Ordering::SeqCst);
            let progress = loaded_assets as f32 / total_assets as f32;
            
            // Clear the screen with custom background color
            clear_background(options.background_color);
            
            // Draw title if one is provided
            if let Some(title) = &options.title {
                let title_size = options.title_font_size;
                let title_dim = measure_text(title, None, title_size, 1.0);
                draw_text(
                    title,
                    screen_width() / 2.0 - title_dim.width / 2.0,
                    screen_height() / 3.0,
                    title_size as f32,
                    options.text_color
                );
            }
            
            // Draw progress text
            let progress_text = format!("Loading: {:.0}%", progress * 100.0);
            draw_text(
                &progress_text,
                screen_width() / 2.0 - measure_text(&progress_text, None, options.progress_font_size, 1.0).width / 2.0,
                screen_height() / 2.0,
                options.progress_font_size as f32,
                options.text_color
            );
            
            // Draw loading bar
            let bar_width = screen_width() * 0.6;
            let bar_height = 30.0;
            let bar_x = screen_width() / 2.0 - bar_width / 2.0;
            let bar_y = screen_height() / 2.0 + 40.0;
            
            // Background bar
            draw_rectangle(bar_x, bar_y, bar_width, bar_height, options.bar_background_color);
            
            // Progress bar
            if progress > 0.0 {
                draw_rectangle(bar_x, bar_y, bar_width * progress, bar_height, options.bar_fill_color);
            }
            
            // Border
            draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, options.text_color);
            
            // Display current file if available
            if loaded_assets > 0 && loaded_assets < total_assets {
                let file_name = assets[loaded_assets].split('/').last().unwrap_or("");
                let file_text = format!("Loading: {}", file_name);
                draw_text(
                    &file_text,
                    screen_width() / 2.0 - measure_text(&file_text, None, options.filename_font_size, 1.0).width / 2.0,
                    bar_y + bar_height + 30.0,
                    options.filename_font_size as f32,
                    options.filename_color
                );
            }
            
            // Check if loading is complete
            if loaded_assets >= total_assets {
                // Show completion message if enabled
                if options.show_completion_message {
                    clear_background(options.background_color);
                    let text_size = options.progress_font_size + 20; // Slightly larger than progress font
                    let text_dimensions = measure_text(&options.completion_message, None, text_size, 1.0);
                    let text_x = screen_width() / 2.0 - text_dimensions.width / 2.0;
                    let text_y = screen_height() / 2.0;
                    
                    draw_text(&options.completion_message, text_x, text_y, text_size as f32, options.text_color);
                    next_frame().await;
                    
                    // Apply completion delay if specified
                    if options.completion_delay > 0.0 {
                        let start_time = get_time();
                        while get_time() - start_time < options.completion_delay as f64 {
                            next_frame().await;
                        }
                    }
                }
                
                // Break the loading loop and proceed with the game
                break;
            }
            
            // Update the screen WITHOUT awaiting asset loading
            next_frame().await;
        }
    }
}
