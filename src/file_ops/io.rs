use crate::models::{DocumentLine, DocumentMetadata, LineElement};
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn content_to_text(lines: &[DocumentLine]) -> String {
    lines
        .iter()
        .map(|line| {
            line.elements
                .iter()
                .map(|element| match element {
                    LineElement::Text(text) => text.clone(),
                    LineElement::Image { id, .. } => format!("[img_load(\"{}\")]", id),
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn text_to_content(
    text: &str,
    metadata: &DocumentMetadata,
    image_data: &mut HashMap<String, Vec<u8>>,
) -> Result<Vec<DocumentLine>, Box<dyn std::error::Error>> {
    let mut lines = Vec::new();

    for line_text in text.lines() {
        if line_text.starts_with("[img_load(\"") && line_text.ends_with("\")]") {
            // Extract image ID
            let start = "[img_load(\"".len();
            let end = line_text.len() - "\")]".len();
            let id = &line_text[start..end];

            if let Some(image_meta) = metadata.images.get(id) {
                let image_element = LineElement::Image {
                    id: id.to_string(),
                    width: image_meta.width,
                    height: image_meta.height,
                };

                let line = DocumentLine {
                    elements: vec![image_element],
                };
                lines.push(line);

                // Decode and store image data if not already present
                if !image_data.contains_key(id) {
                    let image_bytes = general_purpose::STANDARD.decode(&image_meta.data)?;
                    image_data.insert(id.to_string(), image_bytes);
                }
            } else {
                // Fallback to text line
                let line = DocumentLine {
                    elements: vec![LineElement::Text(line_text.to_string())],
                };
                lines.push(line);
            }
        } else {
            let line = DocumentLine {
                elements: vec![LineElement::Text(line_text.to_string())],
            };
            lines.push(line);
        }
    }

    if lines.is_empty() {
        lines.push(DocumentLine::new());
    }

    Ok(lines)
}

pub fn save_to_path(
    path: PathBuf,
    lines: &[DocumentLine],
    metadata: &DocumentMetadata,
) -> Result<(), Box<dyn std::error::Error>> {
    let text_content = content_to_text(lines);

    // Determine file extension
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("txt");

    match extension {
        "md" | "txt" => {
            // Save as plain text with image placeholders
            fs::write(&path, text_content)?;

            // Save metadata file if there are images
            if !metadata.images.is_empty() {
                let metadata_path = path.with_extension(format!("{}.meta", extension));
                let metadata_json = serde_json::to_string_pretty(metadata)?;
                fs::write(metadata_path, metadata_json)?;
            }
        }
        _ => {
            // Default to txt format
            fs::write(&path, text_content)?;
            if !metadata.images.is_empty() {
                let metadata_path = path.with_extension("txt.meta");
                let metadata_json = serde_json::to_string_pretty(metadata)?;
                fs::write(metadata_path, metadata_json)?;
            }
        }
    }

    Ok(())
}

pub fn load_from_path(
    path: PathBuf,
) -> Result<
    (
        Vec<DocumentLine>,
        DocumentMetadata,
        HashMap<String, Vec<u8>>,
    ),
    Box<dyn std::error::Error>,
> {
    // Read the text content
    let text_content = fs::read_to_string(&path)?;

    // Try to load metadata file
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("txt");
    let metadata_path = path.with_extension(format!("{}.meta", extension));

    let metadata = if metadata_path.exists() {
        let metadata_json = fs::read_to_string(metadata_path)?;
        serde_json::from_str(&metadata_json)?
    } else {
        DocumentMetadata {
            images: HashMap::new(),
        }
    };

    let mut image_data = HashMap::new();

    // Parse content
    let lines = text_to_content(&text_content, &metadata, &mut image_data)?;

    Ok((lines, metadata, image_data))
}
