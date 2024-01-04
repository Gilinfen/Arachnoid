import pandas as pd
import os
from openpyxl import Workbook
from openpyxl.worksheet.datavalidation import DataValidation

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

        # 创建带有下拉列表的数据验证对象
        dv = DataValidation(type="list", formula1='"隐藏,不隐藏"', allow_blank=True)
        
        # 获取工作表
        worksheet = workbook.active

        # 定义应用下拉列表的单元格范围
        dv_range = f'I2:I{len(df) + 1}'
        dv.add(dv_range)

        # 将数据验证添加到工作表
        worksheet.add_data_validation(dv)

    # 保存文件
    workbook.save(output_file)
    return output_file

