use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::{Read, Write},
};

use encryption::aes::AesKey;
use rust_extensions::date_time::DateTimeAsMicroseconds;

const FILE_NAME: &str = ".my-secrets-data";

pub fn compile_full_filename(file_name: &str) -> String {
    format!(
        "{}/.my-secrets/{}",
        std::env::var("HOME").unwrap(),
        file_name
    )
}

pub fn load_file(aes_key: &AesKey) -> Option<BTreeMap<String, BTreeMap<String, String>>> {
    let filename = compile_full_filename(FILE_NAME);
    let file_result = File::open(filename);

    if let Err(err) = file_result {
        println!("Can not open file: {}", err);
        return Some(BTreeMap::new());
    }

    let mut file = file_result.unwrap();

    let mut encrypted = Vec::new();

    file.read_to_end(&mut encrypted).unwrap();

    let json = match aes_key.decrypt(&encrypted) {
        Ok(result) => result,
        Err(_) => return None,
    };

    let categories: Result<BTreeMap<String, BTreeMap<String, String>>, _> =
        serde_json::from_slice(&json);

    match categories {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

pub fn save_to_file(aes_key: &AesKey, categories: &BTreeMap<String, BTreeMap<String, String>>) {
    let filename = compile_full_filename(FILE_NAME);
    save_current_as_backup(&filename);
    let mut file = File::create(filename).unwrap();
    let json = serde_json::to_vec(categories).unwrap();

    let encrypted = aes_key.encrypt(&json);

    file.set_len(0).unwrap();
    file.write_all(&encrypted).unwrap();
}

pub fn save_current_as_backup(file_name: &str) {
    let read_file_content = std::fs::read(file_name);

    match read_file_content {
        Ok(content) => {
            //let folder_to_create = compile_full_filename("backups");

            let mut icloud_docs_path = format!(
                "{}/Library/Mobile Documents/com~apple~CloudDocs/backups/my-secrets",
                std::env::var("HOME").unwrap()
            );

            if !std::path::Path::new(icloud_docs_path.as_str()).exists() {
                fs::create_dir(icloud_docs_path.as_str()).unwrap();
            }

            icloud_docs_path.push_str("/");

            icloud_docs_path
                .push_str(&DateTimeAsMicroseconds::now().to_rfc3339().replace(":", "-")[..19]);
            let res = fs::write(icloud_docs_path.as_str(), content);

            if let Err(err) = res {
                println!("Can not write file: {}. Err: {}", icloud_docs_path, err);
            }
        }
        Err(err) => {
            println!("Can not read file: {}. Err: {}", file_name, err);
        }
    }
}
