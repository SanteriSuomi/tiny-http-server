// Guess the mime type of a file based on its extension
pub fn guess_mime_type(extension: &str) -> String {
    String::from(match extension.to_lowercase().as_str() {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        _ => "text/plain",
    })
}
