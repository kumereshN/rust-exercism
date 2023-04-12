pub fn verse(n: u32) -> String {
    match n {
        0 => {
            "No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
        },
        1 => {
            format!("{} bottle of beer on the wall, {} bottle of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n",n,n)
        },
        2 => {
            format!("{} bottles of beer on the wall, {} bottles of beer.\n\
            Take one down and pass it around, {} bottle of beer on the wall.\n",n,n,n-1)
        },
        3..=99 => {
            format!("{} bottles of beer on the wall, {} bottles of beer.\n\
            Take one down and pass it around, {} bottles of beer on the wall.\n",n,n,n-1)
        },
        _ => panic!("Invalid number of bottles: {}",n)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<String>>()
        .join("\n")
}
