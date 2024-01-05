use std::fs;
use walkdir::{DirEntry, WalkDir};
use xlsxwriter::prelude::*;

pub fn create_excel(
    headers: Vec<&str>,
    data: Vec<Vec<String>>,
    filename: &str,
) -> Result<(), XlsxError> {
    let workbook = Workbook::new(filename)?;
    let mut sheet1 = workbook.add_worksheet(None)?;

    // Write headers
    for (i, header) in headers.iter().enumerate() {
        sheet1.write_string(0, i as u16, header, None)?;
    }

    // Write data rows
    for (row_num, row_data) in data.iter().enumerate() {
        for (col_num, cell) in row_data.iter().enumerate() {
            sheet1.write_string(((row_num + 1) as u16).into(), col_num as u16, cell, None)?;
        }
    }

    workbook.close()
}

fn create_data_array(base_path: &str) -> Vec<Vec<String>> {
    let mut data = Vec::new();

    for entry in WalkDir::new(base_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if is_directory(&entry) {
            let folder_path = entry.path().to_string_lossy().to_string();
            let folder_name = entry.file_name().to_string_lossy().to_string();
            let description_path = entry.path().join("Description.txt");
            let description = if description_path.exists() {
                fs::read_to_string(description_path).unwrap_or_default()
            } else {
                String::new()
            };

            let row = vec![
                // 图片路径
                folder_path,
                // 标题
                folder_name,
                // 价格
                String::new(),
                // 类目
                String::new(),
                // 状况
                String::new(),
                // 描述
                description,
                // 存货状况（可空）
                String::new(),
                // 地点（可空）
                String::new(),
                // 对朋友隐藏
                String::new(),
            ];
            data.push(row);
        }
    }

    data
}

fn is_directory(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn update_data_array(data_array: &mut Vec<Vec<String>>, values: Vec<&str>) {
    for row in data_array.iter_mut() {
        for (i, value) in values.iter().enumerate() {
            if i < row.len() {
                row[i + 2] = value.to_string();
            }
        }
    }
}

#[tauri::command]
pub fn create_xlsx(base_path: &str, filename: &str) {
    let headers = vec![
        "图片路径",
        "标题",
        "价格",
        "类目",
        "状况",
        "描述",
        "存货状况（可空）",
        "地点（可空）",
        "对朋友隐藏",
    ];

    let mut data_array: Vec<Vec<String>> = create_data_array(base_path);

    let values = vec!["1", "其他", "全新", "", "", "", "隐藏"];
    update_data_array(&mut data_array, values);

    let _ = create_excel(headers, data_array, filename);
}
