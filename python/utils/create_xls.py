import pandas as pd
import os
from openpyxl import Workbook
from openpyxl.worksheet.datavalidation import DataValidation

def add_data_validation(worksheet, validation_type, formula, column, df_length):
    # Create data validation object
    data_validation = DataValidation(type=validation_type, formula1=formula, allow_blank=True)
    
    # Define the range for the data validation
    range_to_apply = f'{column}2:{column}{df_length + 1}'

    # Add data validation to the worksheet
    data_validation.add(range_to_apply)
    worksheet.add_data_validation(data_validation)

def create_xls_with_dropdown(base_path,target_path):
    # 定义数据框架的列
    columns = ["图片路径", "标题", "价格", "类目", "状况", "描述", "存货状况", "地点", "对朋友隐藏"]
    data = []

    # 遍历基础路径中的目录
    for folder in os.listdir(base_path):
        folder_path = os.path.join(base_path, folder)

        # 检查是否为目录
        if os.path.isdir(folder_path):
            # 如果存在，则从Description.txt中读取描述
            description_file_path = os.path.join(folder_path, "Description.txt")
            description = ""
            if os.path.isfile(description_file_path):
                with open(description_file_path, 'r', encoding='utf-8') as file:
                    description = file.read()

            # 准备行数据
            row = {
                "图片路径": folder_path, 
                "标题": folder, 
                "价格": "", 
                "类目": "", 
                "状况": "", 
                "描述": description, 
                "存货状况（可选）": "", 
                "地点（可选）": "", 
                "对朋友隐藏": ""
            }
            data.append(row)

    # 创建数据框架
    df = pd.DataFrame(data, columns=columns)

    # 使用Openpyxl将数据框架保存为Excel文件
    output_file = os.path.join(target_path, "output_with_dropdown.xlsx")
    with pd.ExcelWriter(output_file, engine='openpyxl') as writer:
        df.to_excel(writer, index=False)
        workbook = writer.book
        
        # 获取工作表
        worksheet = workbook.active

        # Sample data frame length (replace with actual df length)
        df_length = len(df)  # Replace with the actual length of your dataframe

        # Lists for data validation
        hide_list = '"隐藏,不隐藏"'
        condition_list = '"全新,二手 - 近全新,二手 - 良好,二手 - 普通"'
        category_list = '"手表,其他"'

        # Adding data validations
        add_data_validation(worksheet, "list", hide_list, 'I', df_length)
        add_data_validation(worksheet, "list", condition_list, 'E', df_length)
        add_data_validation(worksheet, "list", category_list, 'D', df_length)


    # 保存文件
    workbook.save(output_file)
    return output_file

