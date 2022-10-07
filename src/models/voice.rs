#[derive(Debug, Clone, PartialEq)]
pub struct Voice {
    pub index: usize,
    pub id: &'static str,
    pub name: &'static str,
    pub filename: &'static str,
    pub is_playing: bool,
}

pub const VOICES: [Voice; 4] = [
    Voice {
        index: 0,
        id: "JoyDivision",
        name: "JOY DIVISION",
        filename: "wav/joydivision.wav",
        is_playing: false,
    },
    Voice {
        index: 1,
        id: "Joy",
        name: "JOY",
        filename: "wav/joy.wav",
        is_playing: false,
    },
    Voice {
        index: 2,
        id: "Divi",
        name: "DIVI",
        filename: "wav/divi.wav",
        is_playing: false,
    },
    Voice {
        index: 3,
        id: "John",
        name: "JOHN",
        filename: "wav/john.wav",
        is_playing: false,
    },
];
