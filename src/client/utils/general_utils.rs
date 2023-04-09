pub fn is_static_file(file_extension: &str) -> bool {
    match file_extension.to_lowercase().as_str() {
        "html" | "css" | "js" | "png" | "jpg" | "jpeg" | "gif" | "ico" => true,
        _ => false,
    }
}
