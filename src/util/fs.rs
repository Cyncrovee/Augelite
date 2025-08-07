use std::{fs::File, path::PathBuf};

use super::model::AugeliteState;

pub fn save_file(main_struct: &mut AugeliteState) {
    if let Some(path) = &main_struct.file_path {
        if PathBuf::from(path).exists() {
            std::fs::write(path, main_struct.buffer.clone().finish().to_string()).unwrap()
        } else {
            File::create(path).unwrap();
        }
    }
}
