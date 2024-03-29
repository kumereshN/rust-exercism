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

const SHARP_CHROMATIC_SCALES: &[&str; 12] = &["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
const FLAT_CHROMATIC_SCALES: &[&str; 12] = &["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];
const NO_SHARPS_FLATS: &[&str; 2] = &["C", "a"];
const SHARPS: &[&str; 11] = &["G", "D", "A", "E", "B", "F# major e", "b", "f#", "c#", "g#", "d# minor"];
const FLATS: &[&str; 11] = &["F", "Bb", "Eb", "Ab", "Db", "Gb major d", "g", "c", "f", "bb", "eb minor"];

#[derive(Debug)]
pub struct Error(String);


pub struct Scale<'a>{
    tonic: &'a str,
    intervals: &'a str,
    scale: &'a[&'a str]
}

impl<'a> Scale<'a> {
    pub fn new(tonic: &'a str, intervals: &'a str) -> Result<Scale<'a>, Error> {
        if !tonic.is_empty() {
            match (SHARPS.contains(&tonic), FLATS.contains(&tonic), NO_SHARPS_FLATS.contains(&tonic)) {
                (true, false, false) => {
                    Ok(
                        Scale {
                            tonic: tonic,
                            intervals,
                            scale: SHARP_CHROMATIC_SCALES
                        })
                },
                (false, true, false) => {
                    Ok(
                        Scale {
                            tonic: tonic,
                            intervals,
                            scale: FLAT_CHROMATIC_SCALES
                        }
                    )
                },
                (false, false, true) => {
                    Ok(
                        Scale {
                            tonic: tonic,
                            intervals,
                            scale: SHARP_CHROMATIC_SCALES
                        })
                },
                (false, false, false) => {
                    Ok(
                        Scale {
                            tonic: tonic,
                            intervals,
                            scale: FLAT_CHROMATIC_SCALES
                        })
                },
                _ => Err(Error("Something went wrong".to_string()))
            }
        } else {
            Err(Error("Something went wrong".to_string()))
        }
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let scale = self.scale;
        let scale_len = scale.len();
        let tonic: String = self.tonic
            .chars()
            .enumerate()
            .map(|(i, c)|{
                if i == 0 {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            })
            .collect();

        let tonic_position = scale.iter().position(|&n| n == tonic).unwrap();

        if self.intervals.is_empty() {
            scale
                .iter()
                .cycle()
                .skip(tonic_position)
                .take(scale_len+1)
                .map(|&c| c.to_string())
                .collect()
        } else {
            let mut scale_iter = scale
                .iter()
                .cycle()
                .skip(tonic_position);

            let mut res: Vec<String> = vec![];
            res.push(scale_iter.next().unwrap().to_string());

            for interval in self.intervals.chars() {
                match interval{
                    'M' => {
                        res.push(scale_iter.nth(1).unwrap().to_string())
                    }
                    'm' => {
                        res.push(scale_iter.next().unwrap().to_string())
                    },
                    'A' => {
                        res.push(scale_iter.nth(2).unwrap().to_string())
                    },
                    _ => panic!("Something went wrong")
                }
            }
            res
        }
    }
}
