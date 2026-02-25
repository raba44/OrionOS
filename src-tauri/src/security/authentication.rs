use sha2::{Sha256, Digest};

pub struct Authentication {
    username: String,
    password_hash: String,
}

impl Authentication {
    pub fn new(username: String, password: String) -> Self {
        let hash = format!("{:x}", Sha256::digest(password.as_bytes()));
        Authentication {
            username,
            password_hash: hash,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        let hash = format!("{:x}", Sha256::digest(password.as_bytes()));
        hash == self.password_hash
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}