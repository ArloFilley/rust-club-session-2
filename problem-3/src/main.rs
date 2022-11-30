///////////////////
// 3. Palindrome //
///////////////////

fn is_palindrome(s: &str) -> bool {
    //! Checks whether a string is a palindrome

    // Rust doesn't let you index into `&str` in traditional ways.
    // This converts it to a `&[u8]`, an array of bytes
    let s = s.as_bytes();

    unimplemented!();
}

fn main() {
    let string = "abcdefedcba";
    let result = is_palindrome(string);

    println!("The string is a palindrome: {}", result);

    assert_eq!(result, true, "The check was not correct");
}
