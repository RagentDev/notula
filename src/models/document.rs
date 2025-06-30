#[derive(Clone, Debug)]
pub enum LineElement {
    Text(String),
    Image { id: String, width: u32, height: u32 },
}

#[derive(Clone, Debug)]
pub struct DocumentLine {
    pub elements: Vec<LineElement>,
}

impl DocumentLine {
    pub fn new() -> Self {
        Self {
            elements: vec![LineElement::Text(String::new())],
        }
    }

    pub fn text_content(&self) -> String {
        self.elements
            .iter()
            .filter_map(|e| match e {
                LineElement::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn is_empty(&self) -> bool {
        self.elements.iter().all(|e| match e {
            LineElement::Text(text) => text.is_empty(),
            _ => false,
        })
    }
}
