pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A'{
        vec!["A".to_string()]
    }
    else {
        let alphabets = ('A'..=c).collect::<Vec<char>>();
        let reverse_alphabets = alphabets.iter().rev().copied().collect::<Vec<char>>();
        let alphabets_len = alphabets.len();

        let empty_space = "".to_string();
        let mut max_internal_white_space = (1..).step_by(2).nth(alphabets_len-2).unwrap();
        let range_of_internal_whitespace=  (1..=max_internal_white_space).rev().step_by(2).collect::<Vec<usize>>();

        let first_half = reverse_alphabets
            .iter()
            .enumerate()
            .map(|(i,&c)| {
                match i + 1 == alphabets_len {
                    true => {
                        format!("{:^width$}", c, width = 2 + max_internal_white_space)
                    },
                    false => {
                        format!("{empty_space:>external_width$}{c:<internal_width$}{c}{empty_space:<external_width$}",
                                external_width = i,
                                internal_width = range_of_internal_whitespace.get(i).unwrap() + 1
                        )
                    }
                }
            })
            .rev();

        let second_half = first_half.clone()
            .rev()
            .skip(1);

        first_half
            .chain(second_half)
            .collect::<Vec<String>>()
    }

}
