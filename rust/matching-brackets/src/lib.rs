pub fn brackets_are_balanced(string: &str) -> bool {
    let mut temp_stack: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => temp_stack.push(c),
            ')' => {
                if temp_stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if temp_stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if temp_stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => (),
        }
    }

    temp_stack.is_empty()
}
