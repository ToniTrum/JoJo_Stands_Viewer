use gpui::*;

// #[derive(IntoElement)]
pub struct StandImage {
    stand_name: String
}

impl StandImage {
    pub fn new(stand_name: String) -> Self {
        Self {
            stand_name
        }
    }
}

