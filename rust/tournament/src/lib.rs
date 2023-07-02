use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut heading = "Team                           | MP |  W |  D |  L |  P".to_string();

    if match_results.is_empty() {
        return heading
    }

    let split_match_results = match_results
                                                    .lines()
                                                    .flat_map(|l| {
                                                        l.split(';')
                                                    })
                                                    .collect::<Vec<&str>>();

    let mut counter_hmap: HashMap<&str, Vec<u8>> = HashMap::new();

    split_match_results
        .chunks(3)
        .for_each(|team| {
            let (team1, team2, condition) = (team[0], team[1], team[2]);
            counter_hmap.entry(team1).or_insert(vec![0u8;5]);
            counter_hmap.entry(team2).or_insert(vec![0u8;5]);

            match condition {
                /*
                Have to change the format!() position,
                at the end sort it out according to the points
                the team scored*/
                 "win" => {
                    if let Some(v) = counter_hmap
                        .get_mut(team1) {
                            v[0] += 1;
                            v[1] += 1;
                            v[4] += 3
                        }

                     if let Some(v) = counter_hmap
                         .get_mut(team2) {
                         v[0] += 1;
                         v[3] += 1
                     }
                },
                "loss" => {

                    if let Some(v) = counter_hmap
                        .get_mut(team1) {
                        v[0] += 1;
                        v[3] += 1;
                    }

                    if let Some(v) = counter_hmap
                        .get_mut(team2) {
                        v[0] += 1;
                        v[1] += 1;
                        v[4] += 3;
                    }
                },
                "draw" => {
                    if let Some(v) = counter_hmap
                        .get_mut(team1) {
                        v[0] += 1;
                        v[2] += 1;
                        v[4] += 1;
                    }

                    if let Some(v) = counter_hmap
                        .get_mut(team2) {
                        v[0] += 1;
                        v[2] += 1;
                        v[4] += 1;
                    }
                },
                _ => panic!("Unknown condition")
            }
        });

    // Extract the entries into a vector
    let mut entries: Vec<(&&str, &Vec<u8>)> = counter_hmap.iter().collect();

    // Sort the vector based on the 4th index of the Vec<u8>
    entries.sort_by(|&(k1, v1), &(k2, v2)| {
        let cmp = v2.get(3).cmp(&v1.get(3));
        if cmp == std::cmp::Ordering::Equal {
            k1.cmp(k2)
        }
        else {
            cmp.reverse()
        }
    });


    for (&team, value) in entries.iter() {
        heading.push_str(
            format!(
                "\n{team}             |  {} |  {} |  {} |  {} |  {}",
                value[0],
                value[1],
                value[2],
                value[3],
                value[4]
            ).to_string().as_str()
        )
    }

    heading
}
