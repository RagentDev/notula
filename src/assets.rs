// src/assets.rs
use crate::icons::Icons;

pub struct AssetManager {
    pub icons: Icons,
}

impl Default for AssetManager {
    fn default() -> Self {
        Self {
            icons: Icons::default(),
        }
    }
}

impl AssetManager {
    pub fn new() -> Self {
        Self::default()
    }
}