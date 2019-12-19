pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];
    string.chars().all(|c| match c {
        '{' => {
            stack.push('}');
            true
        }
        '[' => {
            stack.push(']');
            true
        }
        '(' => {
            stack.push(')');
            true
        }
        c @ '}' | c @ ']' | c @ ')' => Some(c) == stack.pop(),
        _ => true,
    }) && stack.is_empty()
}
