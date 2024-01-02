use std::fs::File;

use base64::{engine::general_purpose, Engine as _};
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Sign, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tauri::AppHandle;

use super::{
    lib::{get_app_data_dir, read_json, unzip_python, update_json, ACCREDIT_PUBLIC_KEY_PEM},
    window,
};

fn get_activate_path(app_handle: &AppHandle) -> String {
    let data_path = get_app_data_dir(&app_handle);
    data_path
        .join("activate.json")
        .to_string_lossy()
        .to_string()
}

// 使用公钥验证签名
pub fn verify_signature(data: &str, signature: &str) -> bool {
    let vec_data = data.as_bytes().to_vec();
    let vec_signature = signature.as_bytes().to_vec();

    // let pub_key_path = get_resource_path(app_handle, "../public_key.pem");
    // let pub_key = RsaPublicKey::read_public_key_pem_file(pub_key_path).expect("读取公钥失败");

    let public_key_pem = ACCREDIT_PUBLIC_KEY_PEM;
    // let public_key_pem_code =
    //     env::var("ACCREDIT_PUBLIC_KEY_PEM").expect("ACCREDIT_PUBLIC_KEY_PEM not set");

    // let public_key_pem2 = decode_str(public_key_pem_code);

    // match env::var("ACCREDIT_PUBLIC_KEY_PEM") {
    //     Ok(val) => println!("ACCREDIT_PUBLIC_KEY_PEM: {}", val),
    //     Err(e) => println!("Couldn't read ACCREDIT_PUBLIC_KEY_PEM ({})", e),
    // }

    // 从字符串中解析公钥
    let pub_key = RsaPublicKey::from_public_key_pem(&public_key_pem).expect("解析公钥失败");

    let dencoed = general_purpose::STANDARD_NO_PAD.decode(&vec_signature);
    match dencoed {
        Ok(dencoed_val) => {
            let mut hasher = Sha256::new(); // 创建 SHA-256 哈希实例
            hasher.update(vec_data); // 对数据进行哈希处理
            let hashed_data = hasher.finalize();

            // 验证签名
            pub_key
                .verify(Pkcs1v15Sign::new_unprefixed(), &hashed_data, &dencoed_val)
                .is_ok()
        }
        Err(_) => false,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyData {
    pub user_info: String,
    pub signature: String,
}

#[tauri::command]
pub async fn use_verify_signature(
    app_handle: AppHandle,
    data: &str,
    signature: &str,
) -> Result<bool, String> {
    let verify_bool = verify_signature(&data, &signature);
    if verify_bool {
        let data_path_str = get_activate_path(&app_handle);
        let ver_data = VerifyData {
            user_info: data.to_string(),
            signature: signature.to_string(),
        };

        // 保存数据
        let _ = File::create(&data_path_str);
        let _ = update_json(&data_path_str, &ver_data);

        // 解压 python
        let _ = unzip_python().await;

        window::app_ready(app_handle.clone());
    }
    Ok(verify_bool)
}

pub fn get_verify_signature(app_handle: &AppHandle) -> bool {
    let data_path_str = get_activate_path(&app_handle);

    let verify_json = read_json::<VerifyData>(&data_path_str);
    match verify_json {
        Ok(p) => verify_signature(&p.user_info, &p.signature),
        Err(e) => {
            println!("{:?}", e);
            false
        }
    }
}
