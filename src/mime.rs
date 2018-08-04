// @see https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Complete_list_of_MIME_types
pub fn from_filename(filename: &str) -> String {
    let mut mime = "application/octet-stream";
    let parts: Vec<&str> = filename.rsplitn(2, ".").collect();
    if !parts.is_empty() {
        if let Some(extension) = parts.get(0) {
            let extension: String = extension.to_lowercase();
            // TODO Support all extensions here
            mime = match extension.as_ref() {
                "css" => "text/css",
                "html" => "text/html",
                "htm" => "text/html",
                "jpeg" => "image/jpeg",
                "jpg" => "image/jpg",
                "js" => "application/javascript",
                "json" => "application/json",
                "png" => "image/png",
                _ => "application/octet-stream"
            };
        }
    }
    mime.to_string()
}
