use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub id: String,
    pub data: String, // base64 encoded image data
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub images: HashMap<String, ImageMetadata>,
}