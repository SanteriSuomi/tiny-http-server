// Guess the mime type of a file based on its extension
pub fn guess_mime_type(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        "html" => String::from("text/html"),
        "css" => String::from("text/css"),
        "js" => String::from("text/javascript"),
        "png" => String::from("image/png"),
        "jpg" | "jpeg" => String::from("image/jpeg"),
        "gif" => String::from("image/gif"),
        "ico" => String::from("image/x-icon"),
        _ => String::from("text/plain"),
    }
}
