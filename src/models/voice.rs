#[derive(Debug, Clone, PartialEq)]
pub struct Voice {
    pub id: &'static str,
    pub name: &'static str,
    pub filename: &'static str,
}

pub const VOICES: [Voice; 4] = [
    Voice {
        id: "JoyDivision",
        name: "Joy Division",
        filename: "wav/joydivision.wav",
    },
    Voice {
        id: "Joy",
        name: "Joy",
        filename: "wav/joy.wav",
    },
    Voice {
        id: "Divi",
        name: "Divi",
        filename: "wav/divi.wav",
    },
    Voice {
        id: "John",
        name: "John",
        filename: "wav/john.wav",
    },
];
