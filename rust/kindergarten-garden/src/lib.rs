const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    let cup_idx = CHILDREN
        .iter()
        .position(|&s| s == _student)
        .unwrap() * 2;

    _diagram
        .lines()
        .flat_map(|line| {
            line[cup_idx..=cup_idx+1]
                .chars()
                .map(|cup| match cup {
                    'G' => "grass",
                    'C' => "clover",
                    'R' => "radishes",
                    _ => "violets",
                })
        })
        .collect()
}
