#[derive(Debug)]
pub struct Error {
    message: String,
}

pub struct Scale {
    notes: Vec<String>,
}

const SHARP: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLAT: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

fn get_scale_type(tonic: &str) -> Result<&[&str], Error> {
    match tonic {
        _ if [
            "C", "a", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#", "a#",
        ]
        .contains(&tonic) =>
        {
            Ok(&SHARP)
        }

        _ if [
            "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
        ]
        .contains(&tonic) =>
        {
            Ok(&FLAT)
        }

        _ => Err(Error {
            message: "Invalid tonic".into(),
        }),
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let scale_type = get_scale_type(tonic)?;

        let mut index = scale_type
            .iter()
            .position(|&x| x.to_uppercase() == tonic.to_uppercase())
            .expect("Invalid tonic");
        let mut notes: Vec<String> = vec![scale_type[index].into()];

        for interval in intervals.chars() {
            let shift_right = match interval {
                'A' => 3,
                'M' => 2,
                'm' => 1,
                _ => {
                    return Err(Error {
                        message: "Invalid interval".into(),
                    })
                }
            };
            index += shift_right;
            let note = scale_type[index % scale_type.len()].into();
            notes.push(note);
        }

        Ok(Scale { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.notes.clone()
    }
}
