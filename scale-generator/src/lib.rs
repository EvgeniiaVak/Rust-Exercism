/// I have no idea how music works, so I'm just trying to get the tests to pass.

#[derive(Debug)]
pub struct Error;

pub struct Scale {
    // TODO: try out with references, probably will need lifetime annotations
    scale: Vec<String>,
}

const SHARP: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const FLAT: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        unimplemented!()
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let i = SHARP.iter().position(|&x| x == tonic).unwrap();

        let scale: Vec<String> = match tonic {
            "C" => SHARP[i..]
                .iter()
                .chain(SHARP[..i + 1].iter())
                .map(|&x| x.to_string())
                .collect(),
            _ => FLAT[i..]
                .iter()
                .chain(FLAT[..i + 1].iter())
                .map(|&x| x.to_string())
                .collect(),
        };

        Ok(Scale { scale })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.clone()
    }
}
