use crate::models::{DocumentLine, LineElement, ImageMetadata};
use image::{DynamicImage, GenericImageView};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;

pub fn create_sample_image() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Create a simple colored rectangle as a sample image
    let width = 200u32;
    let height = 100u32;
    let mut img_buffer = image::RgbaImage::new(width, height);
    
    // Fill with a gradient
    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let r = (x as f32 / width as f32 * 255.0) as u8;
        let g = (y as f32 / height as f32 * 255.0) as u8;
        let b = 128u8;
        *pixel = image::Rgba([r, g, b, 255]);
    }
    
    let dynamic_img = DynamicImage::ImageRgba8(img_buffer);
    let mut buffer = Vec::new();
    let _ = dynamic_img.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png);
    Ok(buffer)
}

pub fn get_image_from_clipboard() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    println!("Attempting to paste image from clipboard...");
    let mut clipboard = arboard::Clipboard::new()?;
    let image_data = clipboard.get_image()?;
    
    println!("Got image data: {}x{}", image_data.width, image_data.height);
    
    let mut buffer = Vec::new();
    let img = image::RgbaImage::from_raw(
        image_data.width as u32,
        image_data.height as u32,
        image_data.bytes.into_owned(),
    ).ok_or("Failed to create image from clipboard data")?;
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    let mut cursor = std::io::Cursor::new(&mut buffer);
    dynamic_img.write_to(&mut cursor, image::ImageOutputFormat::Png)?;
    
    println!("Created PNG buffer of {} bytes", buffer.len());
    Ok(buffer)
}

pub fn process_image_bytes(
    image_bytes: Vec<u8>, 
    lines: &mut Vec<DocumentLine>,
    cursor_line: &mut usize,
    cursor_col: &mut usize,
    image_data: &mut HashMap<String, Vec<u8>>,
    metadata_images: &mut HashMap<String, ImageMetadata>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing image from {} bytes", image_bytes.len());
    let img = image::load_from_memory(&image_bytes)?;
    let (width, height) = img.dimensions();
    let id = Uuid::new_v4().to_string();
    
    println!("Created image ID: {}, dimensions: {}x{}", id, width, height);
    
    // Store image data and metadata
    image_data.insert(id.clone(), image_bytes.clone());
    let image_metadata = ImageMetadata {
        id: id.clone(),
        data: general_purpose::STANDARD.encode(&image_bytes),
        width,
        height,
    };
    metadata_images.insert(id.clone(), image_metadata);
    
    let image_element = LineElement::Image {
        id: id.clone(),
        width,
        height,
    };
    
    // Insert image on a new line after the current cursor position
    let new_line = DocumentLine {
        elements: vec![image_element],
    };
    
    *cursor_line += 1;
    lines.insert(*cursor_line, new_line);
    *cursor_col = 0;
    
    // Add an empty line after the image for continued typing
    *cursor_line += 1;
    lines.insert(*cursor_line, DocumentLine::new());
    
    println!("Inserted image on line {}. Total lines: {}", *cursor_line - 1, lines.len());
    
    Ok(())
}