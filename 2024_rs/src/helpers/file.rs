use std::{fs, path::Path};

/// Читает файл и возвращает Vec строк
pub fn read_file_lines(file_path: &str) -> Vec<String> {
    let path = Path::new(file_path);

    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

/// Читает файл и возвращает строку
pub fn read_file_as_str(file_path: &str) -> String {
    let path = Path::new(file_path);

    fs::read_to_string(path).unwrap()
}
