pub fn is_palindrome(string : &String) -> bool {
    let cleaned_string = string.to_ascii_lowercase().chars()
        .filter(|letter| letter.is_alphabetic()).collect::<String>();

    cleaned_string == cleaned_string.chars().rev().collect::<String>()
}