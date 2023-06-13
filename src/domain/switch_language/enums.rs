#[derive(Debug)]
pub enum Languages {
    En,
    Ua,
}

impl Languages {
    pub fn as_str(&self) -> &str {
        match self {
            Languages::En => "en",
            Languages::Ua => "ua",
        }
    }
}
