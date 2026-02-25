pub struct Encryption {
    key: String,
}

impl Encryption {
    pub fn new(key: String) -> Self {
        Encryption { key }
    }

    pub fn encrypt(&self, data: &str) -> String {
        format!("encrypted_{}", data)
    }

    pub fn decrypt(&self, encrypted_data: &str) -> String {
        encrypted_data.replace("encrypted_", "")
    }
}