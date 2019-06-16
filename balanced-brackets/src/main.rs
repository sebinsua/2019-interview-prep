fn is_open_bracket(c: char) -> bool {
    let normal_bracket_open = '(';
    let square_bracket_open = '[';
    let curly_bracket_open = '{';

    c == normal_bracket_open || c == square_bracket_open || c == curly_bracket_open
}

fn is_close_bracket(c: char) -> bool {
    let normal_bracket_close = ')';
    let square_bracket_close = ']';
    let curly_bracket_close = '}';

    c == normal_bracket_close || c == square_bracket_close || c == curly_bracket_close
}

fn is_matching_bracket(open_bracket: char, close_bracket: char) -> bool {
    match open_bracket {
        '(' => close_bracket == ')',
        '[' => close_bracket == ']',
        '{' => close_bracket == '}',
        _ => false,
    }
}

/*
 * Given a string write a method to check whether all of its brackets are balanced.
 */
fn has_balanced_brackets(string: &str) -> bool {

    let mut open_brackets: Vec<char> = Vec::new();
    for c in string.chars() {
        if is_open_bracket(c) {
            open_brackets.push(c);
        } else if is_close_bracket(c) {
            if open_brackets.len() == 0 {
                return false;
            }

            let last_open_bracket = open_brackets[open_brackets.len() - 1];
            if is_matching_bracket(last_open_bracket, c) {
                open_brackets.pop();
            } else {
                return false;
            }
        }
    }

    if open_brackets.len() == 0 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {

    use crate::has_balanced_brackets;

    #[test]
    fn can_detect_when_brackets_are_balanced() {
        assert_eq!(true, has_balanced_brackets("()"));
        assert_eq!(true, has_balanced_brackets("[]"));
        assert_eq!(true, has_balanced_brackets("{}"));
        assert_eq!(true, has_balanced_brackets("{}([])"));
        assert_eq!(true, has_balanced_brackets("{[()]}"));
        assert_eq!(true, has_balanced_brackets("{{[[(())]]}}"));
        assert_eq!(true, has_balanced_brackets("{a}[b(c)]"))
    }

    #[test]
    fn can_detect_when_brackets_are_not_balanced() {
        assert_eq!(false, has_balanced_brackets(")"));
        assert_eq!(false, has_balanced_brackets(")("));
        assert_eq!(false, has_balanced_brackets("]["));
        assert_eq!(false, has_balanced_brackets("}{"));
        assert_eq!(false, has_balanced_brackets("{[(])}"));
        assert_eq!(false, has_balanced_brackets("{}[[)]]"));
        assert_eq!(false, has_balanced_brackets("({])}"));
    }

}
