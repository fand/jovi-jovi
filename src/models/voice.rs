#[derive(PartialEq, Clone)]
pub enum Voice {
    JoyDivision,
    Joy,
    Divi,
    John,
}

impl Voice {
    pub fn voices() -> Vec<Voice> {
        vec![Voice::JoyDivision, Voice::Joy, Voice::Divi, Voice::John]
    }
    pub fn name(&self) -> String {
        match self {
            Voice::JoyDivision => "Joy Division".to_string(),
            Voice::Joy => "Joy".to_string(),
            Voice::Divi => "Divi".to_string(),
            Voice::John => "John".to_string(),
        }
    }
}
