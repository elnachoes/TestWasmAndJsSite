use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_palindrome(string : &str) -> bool {
    is_palindrome::is_palindrome(&string.to_string())
}