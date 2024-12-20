use std::{fs, path::Path};

/// Читает файл и возвращает Vec строк
pub fn read_lines(file_path: &str) -> Vec<String> {
    let path = Path::new(file_path);

    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
