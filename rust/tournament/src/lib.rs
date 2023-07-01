use std::collections::HashMap;
use std::iter;

pub fn tally(match_results: &str) -> String {
    let heading = "Team                           | MP |  W |  D |  L |  P".to_string();

    if match_results.is_empty() {
        return heading
    }

    let split_match_results = match_results
                                                    .lines()
                                                    .flat_map(|l| {
                                                        l.split(';')
                                                    })
                                                    .collect::<Vec<&str>>();

    iter::once(heading)
        .chain(
            split_match_results
                .chunks(3)
                .map(|team| {
                    let (team1, team2, condition) = (team[0], team[1], team[2]);
                    let mut counter_hmap = HashMap::from([
                        (team1, vec![0; 5]),
                        (team2, vec![0; 5])
                    ]);
                    match condition {
                        /*
                        Have to change the format!() position,
                        at the end sort it out according to the points
                        the team scored*/
                         "win" => {
                            counter_hmap.entry(team1).and_modify(|v| {
                                v[0] += 1;
                                v[1] += 1;
                                v[4] += 3
                            });

                            counter_hmap.entry(team2).and_modify(|v| {
                                v[0] += 1;
                                v[3] += 1
                            });

                            format!(
                                "\n{team1}             |  {} |  {} |  {} |  {} |  {}\n{team2}             |  {} |  {} |  {} |  {} |  {}",
                                counter_hmap.get(team1).unwrap()[0],
                                counter_hmap.get(team1).unwrap()[1],
                                counter_hmap.get(team1).unwrap()[2],
                                counter_hmap.get(team1).unwrap()[3],
                                counter_hmap.get(team1).unwrap()[4],
                                counter_hmap.get(team2).unwrap()[0],
                                counter_hmap.get(team2).unwrap()[1],
                                counter_hmap.get(team2).unwrap()[2],
                                counter_hmap.get(team2).unwrap()[3],
                                counter_hmap.get(team2).unwrap()[4]
                            )
                        },
                        "loss" => {
                            counter_hmap.entry(team1).and_modify(|v| {
                                v[0] += 1;
                                v[3] += 1;
                            });

                            counter_hmap.entry(team2).and_modify(|v| {
                                v[0] += 1;
                                v[1] += 1;
                                v[4] += 3;
                            });

                            format!(
                                "\n{team2}             |  {} |  {} |  {} |  {} |  {}\n{team1}             |  {} |  {} |  {} |  {} |  {}",
                                counter_hmap.get(team2).unwrap()[0],
                                counter_hmap.get(team2).unwrap()[1],
                                counter_hmap.get(team2).unwrap()[2],
                                counter_hmap.get(team2).unwrap()[3],
                                counter_hmap.get(team2).unwrap()[4],
                                counter_hmap.get(team1).unwrap()[0],
                                counter_hmap.get(team1).unwrap()[1],
                                counter_hmap.get(team1).unwrap()[2],
                                counter_hmap.get(team1).unwrap()[3],
                                counter_hmap.get(team1).unwrap()[4],
                            )
                        },
                        "draw" => {"DRAW".to_string()},
                        _ => panic!("Unknown condition")

                    }
                })
        )
        .collect::<String>()
}
