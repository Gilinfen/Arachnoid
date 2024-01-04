# 处理产品文件

import os
import shutil

# 基于文本文件的内容复制图像和文本文件到新文件夹的功能
def copy_files_to_new_folders(base_path, target_path):
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
                    # Remove "REPLICA ROLEX" from the first line
                    new_folder_name = first_line.replace("REPLICA ROLEX", "").strip()

                    # Create a new directory with the modified first line as its name
                    new_folder_path = os.path.join(target_path, new_folder_name)
                    os.makedirs(new_folder_path, exist_ok=True)

                    # Copy images and the text file to the new directory
                    for file_name in os.listdir(folder_path):
                        file_path = os.path.join(folder_path, file_name)
                        if file_name.lower().endswith(('.png', '.jpg', '.jpeg', '.gif', '.bmp')) or file_name == "新建文本文档.txt":
                            new_file_name = "Product introduction.txt" if file_name == "新建文本文档.txt" else file_name
                            shutil.copy2(file_path, os.path.join(new_folder_path, new_file_name))

