use std::fmt::{Debug, Display};

use sha1::{Digest, Sha1};

#[derive(Clone, Copy)]
pub struct Hash([u8; 20]);

impl Hash {
    pub fn new(hash: [u8; 20]) -> Self {
        Self(hash)
    }

    pub fn bytes(&self) -> [u8; 20] {
        self.0
    }

    pub fn hash(bytes: &[u8]) -> Self {
        let mut hasher = Sha1::new();
        hasher.update(bytes);
        let hash = hasher.finalize();
        Self::new(hash.into())
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Hash").field(&hex::encode(self.0)).finish()
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}
