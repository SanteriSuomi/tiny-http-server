// Guess the mime type of a file based on its extension
pub fn guess_mime_type(extension: &str) -> String {
    let guess = mime_guess::from_ext(extension).first();
    if let Some(guess) = guess {
        String::from(guess.essence_str())
    } else {
        String::from("text/plain")
    }
}
