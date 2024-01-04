import os
import shutil

# Function to copy images and text file to new folders based on the content of the text file
def copy_files_to_new_folders(base_path, target_path, remove_strings):
    # Split the string of words to remove into a list
    words_to_remove = remove_strings.split(',')

    # Iterate over the directories in the base path
    for folder in os.listdir(base_path):
        folder_path = os.path.join(base_path, folder)

        # Check if it is a directory
        if os.path.isdir(folder_path):
            text_file_path = os.path.join(folder_path, "新建文本文档.txt")

            # Check if the text file exists
            if os.path.isfile(text_file_path):
                with open(text_file_path, 'r', encoding='utf-8') as file:
                    first_line = file.readline().strip()

                    # Remove specified strings from the first line
                    for word in words_to_remove:
                        first_line = first_line.replace(word, "").strip()

                    # Create a new directory with the modified first line as its name
                    new_folder_path = os.path.join(target_path, first_line)
                    os.makedirs(new_folder_path, exist_ok=True)

                    # Copy images and the text file to the new directory
                    for file_name in os.listdir(folder_path):
                        file_path = os.path.join(folder_path, file_name)
                        if file_name.lower().endswith(('.png', '.jpg', '.jpeg', '.gif', '.bmp')) or file_name == "新建文本文档.txt":
                            new_file_name = "Product introduction.txt" if file_name == "新建文本文档.txt" else file_name
                            shutil.copy2(file_path, os.path.join(new_folder_path, new_file_name))
