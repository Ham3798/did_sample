use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: String,
    pub private_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct DIDDocument {
    pub context: String,
    pub id: String,
    pub public_key: String,
}

impl KeyPair {
    pub fn generate() -> Self {
        // 여기서 실제 키 쌍을 생성하는 로직을 구현해야 합니다.
        // 예시를 위해 임시 키 값을 반환합니다.
        Self {
            public_key: "public_key_example".into(),
            private_key: "private_key_example".into(),
        }
    }
}
