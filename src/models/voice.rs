#[derive(Debug, Clone, PartialEq)]
pub struct Voice {
    pub index: usize,
    pub id: &'static str,
    pub name: &'static str,
    pub filename: &'static str,
    pub is_playing: bool,
}

pub const VOICES: [Voice; 6] = [
    Voice {
        index: 0,
        id: "Joy",
        name: "JOY",
        filename: "sounds/jovijovi_joy.mp3",
        is_playing: false,
    },
    Voice {
        index: 1,
        id: "Divi",
        name: "DIVI",
        filename: "sounds/jovijovi_divi.mp3",
        is_playing: false,
    },
    Voice {
        index: 2,
        id: "Jon",
        name: "JON",
        filename: "sounds/jovijovi_jon.mp3",
        is_playing: false,
    },
    Voice {
        index: 3,
        id: "Jon?",
        name: "JON?",
        filename: "sounds/jovijovi_jon-.mp3",
        is_playing: false,
    },
    Voice {
        index: 4,
        id: "Bon",
        name: "BON",
        filename: "sounds/jovijovi_bon.mp3",
        is_playing: false,
    },
    Voice {
        index: 5,
        id: "Jovi",
        name: "JOVI",
        filename: "sounds/jovijovi_jovi.mp3",
        is_playing: false,
    },
];
