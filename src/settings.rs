use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModel {
    pub shared_key: String,
}

impl SettingsModel {
    pub fn read() -> Self {
        use std::io::Read;

        let file_name = crate::file::compile_full_filename(".my-secrets.yaml");

        let mut file_result = std::fs::File::open(file_name.as_str());
        if file_result.is_err() {
            panic!("Can not read settings from file: {}", file_name);
        }
        let mut result = Vec::new();
        match file_result.as_mut().unwrap().read_to_end(&mut result) {
            Ok(_) => match serde_yaml::from_slice(&result) {
                Ok(result) => return result,
                Err(err) => panic!("Invalid yaml format of file: {}. Err: {}", file_name, err),
            },
            Err(_) => panic!("Can not read settings from file: {}", file_name),
        }
    }

    pub fn get_iv(&self) -> [u8; 16] {
        let mut result = [0u8; 16];
        let bytes = self.shared_key.as_bytes();
        for i in 0..16 {
            result[i] = bytes[i];
        }
        result
    }
}
