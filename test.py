import os
from PIL import Image

# Set the folder path
folder_path = 'assets/'

# Loop through all files in the folder
for filename in os.listdir(folder_path):
    # Check if the file is a PNG image
    if filename.lower().endswith('.png'):
        # Full path to the image
        img_path = os.path.join(folder_path, filename)
        
        # Open the image file
        img = Image.open(img_path)
        
        # Check if the image has an alpha channel
        if img.mode in ('RGBA', 'LA') or (img.mode == 'P' and 'transparency' in img.info):
            print(f"Processing {filename}...")
            
            # Convert to RGB to remove alpha channel
            rgb_img = Image.new("RGB", img.size, (255, 255, 255))
            if img.mode == 'P':
                img = img.convert('RGBA')
            
            # Paste using the alpha channel as mask
            rgb_img.paste(img, mask=img.split()[3] if img.mode == 'RGBA' else None)
            
            # Save the image without alpha channel
            rgb_img.save(img_path, 'PNG')
            print(f"Removed alpha channel from {filename}")
        else:
            print(f"Skipping {filename} - no alpha channel")

print("Finished processing all PNG files in the assets folder")