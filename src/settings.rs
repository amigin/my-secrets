use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsModel {
    pub shared_key: String,
}

impl SettingsModel {
    pub fn read() -> Self {
        let key_chain = security_framework::os::macos::keychain::SecKeychain::default().unwrap();
        let shared_key = key_chain.find_generic_password("my-secrets", "my-secrets");

        if let Err(err) = &shared_key {
            panic!("Can not read settings from keychain. Err: {}", err);
        }

        let shared_key = shared_key.unwrap();

        Self {
            shared_key: String::from_utf8(shared_key.0.to_vec()).unwrap(),
        }

        /*
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
         */
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
