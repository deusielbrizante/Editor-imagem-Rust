use std::path::Path;

pub fn extract_path(outfile: &String) -> String {
    let path = Path::new(outfile.trim());

    return match path.parent() {
        Some(parent) => parent.to_string_lossy().into_owned(),
        _ => String::new(),
    };
}
