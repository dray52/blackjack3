/*
By: <Your Name Here>
Date: 2025-05-09
Program Details: <Program Description Here>
*/

mod modules;

use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use crate::modules::label::Label;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "blackjack3".to_owned(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Create card deck data
    let mut deck = vec![
        "assets/aceHeart.png",
        "assets/aceDiamond.png",
        "assets/aceClubs.png",
        "assets/aceSpade.png",
        "assets/jackHeart.png",
        "assets/jackDiamond.png",
        "assets/jackClubs.png",
        "assets/jackSpade.png",
        "assets/queenHeart.png",
        "assets/queenDiamond.png",
        "assets/queenClubs.png",
        "assets/queenSpade.png",
        "assets/kingHeart.png",
        "assets/kingDiamond.png",
        "assets/kingClubs.png",
        "assets/kingSpade.png",
        "assets/02clubs.png",
        "assets/02heart.png",
        "assets/02diamond.png",
        "assets/02spade.png",
        "assets/03clubs.png",
        "assets/03heart.png",
        "assets/03diamond.png",        
        "assets/03spade.png",
        "assets/04clubs.png",
        "assets/04heart.png",
        "assets/04diamond.png",        
        "assets/04spade.png",
        "assets/05clubs.png",
        "assets/05heart.png",
        "assets/05diamond.png",        
        "assets/05spade.png",
        "assets/06clubs.png",
        "assets/06heart.png",
        "assets/06diamond.png",        
        "assets/06spade.png",
        "assets/07clubs.png",
        "assets/07heart.png",
        "assets/07diamond.png",        
        "assets/07spade.png",
        "assets/08clubs.png",
        "assets/08heart.png",
        "assets/08diamond.png",        
        "assets/08spade.png",
        "assets/09clubs.png",
        "assets/09heart.png",
        "assets/09diamond.png",        
        "assets/09spade.png",
        "assets/10clubs.png",
        "assets/10heart.png",
        "assets/10diamond.png",        
        "assets/10spade.png",];
    
    // Add backcard to the full list of assets
    let all_assets = [&deck[..], &["assets/backcard.png"]].concat();
    
    // Create the texture manager
    let mut tm = TextureManager::new();
    
    // Setup loading screen
    let total_assets = all_assets.len();
    let mut current_asset = 0;
    
    // Web-friendly asset loading with minimal drawing to prevent flashing
    while current_asset < total_assets {
        // Calculate progress
        let loading_progress = current_asset as f32 / total_assets as f32;
        
        // Always use the same background color
        clear_background(DARKGREEN);
        
        // Draw title
        let title_text = "BLACKJACK";
        let title_size = 60;
        let title_dim = measure_text(title_text, None, title_size, 1.0);
        draw_text(
            title_text,
            screen_width() / 2.0 - title_dim.width / 2.0,
            screen_height() / 3.0,
            title_size as f32,
            WHITE
        );
        
        // Draw progress text
        let loading_text = format!("Loading: {:.0}%", loading_progress * 100.0);
        let text_size = 30;
        let text_dim = measure_text(&loading_text, None, text_size, 1.0);
        let text_x = screen_width() / 2.0 - text_dim.width / 2.0;
        let text_y = screen_height() / 2.0;
        draw_text(&loading_text, text_x, text_y, text_size as f32, WHITE);
        
        // Draw loading bar
        let bar_width = screen_width() * 0.6;
        let bar_height = 30.0;
        let bar_x = screen_width() / 2.0 - bar_width / 2.0;
        let bar_y = text_y + 40.0;
        
        // Draw bar background
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, Color::new(0.0, 0.2, 0.0, 1.0));
        
        // Draw progress
        if loading_progress > 0.0 {
            draw_rectangle(bar_x, bar_y, bar_width * loading_progress, bar_height, GREEN);
        }
        
        // Draw border
        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 2.0, WHITE);
        
        // Draw file name
        let file_name = all_assets[current_asset].split('/').last().unwrap_or("");
        let file_text = format!("Loading: {}", file_name);
        let file_size = 20;
        let file_dim = measure_text(&file_text, None, file_size, 1.0);
        draw_text(
            &file_text,
            screen_width() / 2.0 - file_dim.width / 2.0,
            bar_y + bar_height + 40.0,
            file_size as f32,
            SKYBLUE
        );
        
        // Update screen
        next_frame().await;
        
        // Load the asset
        tm.preload(all_assets[current_asset]).await;
        current_asset += 1;
    }
    
    // All assets are loaded, draw a "complete" message
    clear_background(DARKGREEN);
    let completion_text = "Loading Complete!";
    let text_size = 50;
    let text_dimensions = measure_text(completion_text, None, text_size, 1.0);
    let text_x = screen_width() / 2.0 - text_dimensions.width / 2.0;
    let text_y = screen_height() / 2.0;
    
    draw_text(completion_text, text_x, text_y, text_size as f32, WHITE);
    next_frame().await;
    
    // Short delay so users can see the completion message
    let start_time = get_time();
    while get_time() - start_time < 1.0 {
        next_frame().await;
    }

    let mut show = "assets/backcard.png";
    let mut lblplayer = Label::new("0", 450.0, 275.0, 30);
    let mut lbldealer = Label::new("0", 450.0, 100.0, 30);
    let mut start = TextButton::new(500.0, 400.0, 100.0, 50.0, "Start".to_string(), BLUE, GREEN,30);
    let mut rand_card = TextButton::new(400.0, 400.0, 100.0, 50.0, "Hit".to_string(), BLUE, GREEN,30);
    let mut stand = TextButton::new(300.0, 400.0, 100.0, 50.0, "Stand".to_string(), BLUE, GREEN,30);
    let mut reset = TextButton::new(200.0, 400.0, 100.0, 50.0, "reset".to_string(), BLUE, GREEN,30);
    let mut txtbet = TextInput::new(100.0, 500.0, 300.0, 50.0, 40.0);


txtbet.with_colors(WHITE, RED, BLACK, WHITE);
    rand_card.with_round(15.0);
    rand_card.with_border(RED, 5.0);
    start.with_round(15.0);
    start.with_border(RED, 5.0);
   stand.with_round(15.0);
   stand.with_border(RED, 5.0);
   reset.with_round(15.0);
   reset.with_border(RED, 5.0);
    rand_card.enabled=false;
    stand.enabled=false;
    reset.enabled=false;
    txtbet.set_text("bet");


    let mut pcard5 = StillImage::new(show, 75.0, 150.0, 345.0, 200.0,true,1.0).await;
    let mut pcard4 = StillImage::new(show, 75.0, 150.0, 290.0, 200.0,true,1.0).await;
    let mut pcard3 = StillImage::new(show, 75.0, 150.0, 235.0, 200.0,true,1.0).await;
    let mut pcard2 = StillImage::new(show, 75.0, 150.0, 180.0, 200.0,true,1.0).await;
    let mut pcard1 = StillImage::new(show, 75.0, 150.0, 125.0, 200.0,true,1.0).await;
    let mut dcard1 = StillImage::new(show, 75.0, 150.0, 125.0, 25.0,true,1.0).await;
    let mut dcard2 = StillImage::new(show, 75.0, 150.0, 180.0, 25.0,true,1.0).await;
    let mut dcard3 = StillImage::new(show, 75.0, 150.0, 235.0, 25.0,true,1.0).await;
    let mut dcard4 = StillImage::new(show, 75.0, 150.0, 290.0, 25.0,true,1.0).await;
    let mut dcard5 = StillImage::new(show, 75.0, 150.0, 345.0, 25.0,true,1.0).await;
    let mut pvalue = 0;
    let mut dvalue = 0;
    let mut turn = 3;
    let mut dturn = 2;
    rand::srand(miniquad::date::now() as u64);
    loop {
        

        clear_background(DARKGREEN);
        


        
        if rand_card.click() {
            show = deck.choose().unwrap();
            if turn == 3 {
            pcard3.set_preload(tm.get_preload(show).unwrap());
    
            deck.retain(|&x| x != show);
            println!("{}", show);
            pvalue = playervalue(pvalue, show);
            println!("{}", pvalue);
            turn = 4;
            }
            else if turn == 4 {
            pcard4.set_preload(tm.get_preload(show).unwrap());
            deck.retain(|&x| x != show);
            println!("{}", show);
            pvalue = playervalue(pvalue, show);
            println!("{}", pvalue);
            turn = 5;
            }
            else if turn == 5 {
            pcard5.set_preload(tm.get_preload(show).unwrap());
            deck.retain(|&x| x != show);
            println!("{}", show);
            pvalue = playervalue(pvalue, show);
            println!("{}", pvalue);
            }
            if pvalue > 21 {
                println!("You lose");
                rand_card.enabled = false;
                stand.enabled = false;
                reset.enabled = true;
               
                }lblplayer.set_text(&format!("Player value:\n {}", pvalue));
            
        }
        if start.click() {
            stand.enabled = true;
            rand_card.enabled = true;
            show = deck.choose().unwrap();
           pcard1.set_preload(tm.get_preload(show).unwrap());
            deck.retain(|&x| x != show);
           pvalue = playervalue(pvalue, show);
            show = deck.choose().unwrap();
            pcard2.set_preload(tm.get_preload(show).unwrap());
            deck.retain(|&x| x != show);
            pvalue = playervalue(pvalue, show);
            
            show = deck.choose().unwrap();
            dcard1.set_preload(tm.get_preload(show).unwrap());
            deck.retain(|&x| x != show);
            dvalue = dealervalue(dvalue, show);
            start.enabled = false;  
            println!("player value: {}Dealer value: {}", pvalue, dvalue);
            lbldealer.set_text(&format!("Dealer value:\n {}", dvalue));
            lblplayer.set_text(&format!("Player value:\n {}", pvalue));
        }
        if stand.click(){
            stand.enabled = false;
            rand_card.enabled = false;
           
            while dvalue < 18 {
                show = deck.choose().unwrap();
                deck.retain(|&x| x != show);
                if dturn == 2{
                    
                dcard2.set_preload(tm.get_preload(show).unwrap());
                    dturn = 3;
                }
                else if dturn == 3 {
                    dcard3.set_preload(tm.get_preload(show).unwrap());
                    dturn = 4;
                }
                else if dturn == 4 {
                    dcard4.set_preload(tm.get_preload(show).unwrap());
                    dturn = 5;
                }
                else if dturn == 5 {
                    dcard5.set_preload(tm.get_preload(show).unwrap());
                    
                }
            dvalue = dealervalue(dvalue, show);
      }lbldealer.set_text(&format!("Dealer value:\n {}", dvalue));
            println!("player value: {}Dealer value: {}", pvalue, dvalue);
        if pwincheck(pvalue, dvalue)== true {
            println!("You win!");
        }
        else if dwincheck(pvalue, dvalue)== true {
            println!("You lose!");
        }
        else {
            println!("Draw!");
        }
        reset.enabled = true;


        }
        if reset.click(){
            pcard1.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            pcard2.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            pcard3.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            pcard4.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            pcard5.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            dcard1.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            dcard2.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            dcard3.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            dcard4.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            dcard5.set_preload(tm.get_preload("assets/backcard.png").unwrap());
            pvalue = 0;
            dvalue = 0;
            turn = 3;
            dturn = 3;
            start.enabled = true;
            rand_card.enabled = false;
            stand.enabled = false;
            reset.enabled = false;
            lblplayer.set_text(&format!("Player value:\n {}", pvalue));
            lbldealer.set_text(&format!("Dealer value:\n {}", dvalue));
        }


        


        pcard5.draw();
        pcard4.draw();
        pcard3.draw();
        pcard2.draw();
        pcard1.draw();
        dcard5.draw();
        dcard4.draw();
        dcard3.draw();
        dcard2.draw();
        dcard1.draw();
        lblplayer.draw();
        lbldealer.draw();
        txtbet.draw();
       
        
        next_frame().await;
    }
    
}
fn playervalue(mut pvalue: i32, show: &str) -> i32 {
    
    if show.contains("ace") {
        pvalue += 11;
        if pvalue > 21 {
            pvalue -= 10;
        }
    }
    else if show.contains("jack") || show.contains("queen") || show.contains("king") {
        pvalue += 10;
        
    }
    else {let part = &show[7..9];
        let value = part.parse::<i32>().unwrap();
        pvalue += value;
        
    }
    
pvalue
}
fn dealervalue(mut dvalue: i32, show: &str) -> i32 {
    
    if show.contains("ace") {
        dvalue += 11;
        if dvalue > 21 {
            dvalue -= 10;
        }
    }
    else if show.contains("jack") || show.contains("queen") || show.contains("king") {
        dvalue += 10;
        
    }
    else {let part = &show[7..9];
        let value = part.parse::<i32>().unwrap();
        dvalue += value;
        
    }
    
dvalue
}
fn pwincheck(pvalue: i32, dvalue: i32) -> bool {
    if pvalue > 21 {
        return false
    }
    else if pvalue == 21&& dvalue != 21 {
        return true
    }
    else if dvalue > 21&& pvalue < 22 {
        return true
    }
    else if pvalue > dvalue {
        return true
    }
    else{
        return false
    }
}
fn dwincheck(pvalue: i32, dvalue: i32) -> bool {
    if dvalue > 21 {
        return false
    }
    else if dvalue == 21&& pvalue != 21 {
        return true
    }
    else if pvalue > 21&& dvalue < 22 {
        return true
    }
    else if dvalue > pvalue {
        return true
    }
    else{
        return false
    }
}


