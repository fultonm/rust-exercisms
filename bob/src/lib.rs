const YELL_RESPONSE: &str = "Whoa, chill out!";
const QUESTION_RESPONSE: &str = "Sure.";
const NOTHING_RESPONSE: &str = "Fine. Be that way!";
const OTHER_RESPONSE: &str = "Whatever.";

// Generates Bob's lackadasical responses.
pub fn reply(s: &str) -> String {
    if is_yelling(s) {
        String::from(YELL_RESPONSE)
    } else if is_question(s) {
        String::from(QUESTION_RESPONSE)
    } else if s.contains(char::is_alphanumeric) {
        String::from(OTHER_RESPONSE)
    } else {
        String::from(NOTHING_RESPONSE)
    }
}

fn is_question(s: &str) -> bool {
    let index: Option<usize> = s.rfind('?');
    match index {
        Some(index) => !(&s[index + 1..]).contains(char::is_alphanumeric),
        _ => false,
    }
}

fn is_yelling(s: &str) -> bool {
    s.contains(char::is_alphabetic) && s.to_uppercase() == s 
}