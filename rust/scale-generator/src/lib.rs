// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

const SHARP_CHROMATIC_SCALES: [&str; 12] = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
const FLAT_CHROMATIC_SCALES: [&str; 12] = ["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];
const NO_SHARPS_FLATS: [&str; 2] = ["C", "a"];
const SHARPS: [&str; 11] = ["G", "D", "A", "E", "B", "F# major e", "b", "f#", "c#", "g#", "d# minor"];
const FLATS: [&str; 11] = ["F", "Bb", "Eb", "Ab", "Db", "Gb major d", "g", "c", "f", "bb", "eb minor"];

#[derive(Debug)]
pub struct Error(String);


pub struct Scale{
    tonic: String,
    intervals: String,
    scale: Vec<String>
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        if !tonic.is_empty() {
            Ok(
                Scale {
                    tonic: tonic.to_string(),
                    intervals: intervals.to_string(),
                    scale: vec![]
                }
            )
        } else {
            Err(Error("Something went wrong".to_string()))
        }
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        match (SHARPS.contains(&tonic), FLATS.contains(&tonic)) {
            (true, false) => {
                Ok(
                Scale {
                    tonic: tonic.to_string(),
                    intervals: "".to_string(),
                    scale: vec![]
                })
            },
            (false, true) => {
                let tonic_position = FLAT_CHROMATIC_SCALES.iter().position(|&n| n == tonic).unwrap();

                let first_half_scale_vec = FLAT_CHROMATIC_SCALES
                    .iter()
                    .take(tonic_position+1)
                    .map(|&x| x.to_string());

                let second_half_scale_vec = FLAT_CHROMATIC_SCALES
                    .iter()
                    .skip(tonic_position)
                    .take(FLAT_CHROMATIC_SCALES.len()-tonic_position)
                    .map(|&x| x.to_string());

                let scale_vec: Vec<String> = second_half_scale_vec.chain(first_half_scale_vec).collect();

                Ok(
                Scale {
                    tonic: tonic.to_string(),
                    intervals: "".to_string(),
                    scale: scale_vec
                }
                )
            },
            (false, false) => {
                let tonic_position = SHARP_CHROMATIC_SCALES.iter().position(|&n| n == tonic).unwrap();

                let first_half_scale_vec = SHARP_CHROMATIC_SCALES
                    .iter()
                    .take(tonic_position+1)
                    .map(|&x| x.to_string());

                let second_half_scale_vec = SHARP_CHROMATIC_SCALES
                    .iter()
                    .skip(tonic_position)
                    .take(SHARP_CHROMATIC_SCALES.len()-tonic_position)
                    .map(|&x| x.to_string());

                let scale_vec: Vec<String> = second_half_scale_vec.chain(first_half_scale_vec).collect();

                Ok(
                Scale {
                    tonic: tonic.to_string(),
                    intervals: "".to_string(),
                    scale: scale_vec
                })
            },
            _ => Err(Error("Something went wrong".to_string()))
        }
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.clone()
    }
}
