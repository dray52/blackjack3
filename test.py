import os
from PIL import Image

# Set the folder path and the desired output size
folder_path = 'assets/'
output_size = (75, 150)

# Loop through all files in the folder
for filename in os.listdir(folder_path):
    # Check if the file is an image
    if filename.lower().endswith(('.jpg', '.jpeg', '.png', '.gif', '.bmp')):
        # Open the image file
        img_path = os.path.join(folder_path, filename)
        img = Image.open(img_path)

        # Resize the image
        img = img.resize(output_size)

        # Save the resized image
        img.save(os.path.join(folder_path, f'{filename}'))