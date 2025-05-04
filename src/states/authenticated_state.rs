use std::collections::BTreeMap;

use encryption::aes::AesKey;

pub type TypeContent = BTreeMap<String, BTreeMap<String, String>>;

pub struct AuthenticatedState {
    pub aes_key: AesKey,
    pub content: TypeContent,
}
