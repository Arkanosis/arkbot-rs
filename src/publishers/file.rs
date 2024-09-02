use crate::publishers;

use std::{
    fs,
    io::{
        BufWriter,
        Write,
    },
};

pub struct File {
    path: String,
}

impl File {
    pub fn new(root: &str, name: &str) -> Self {
        Self {
            path: format!("{}/{}", root, name),
        }
    }
}

impl publishers::Publish for File {
    fn publish(&self, titles: &Vec<String>, dump: &str) {
        if let Ok(file) = fs::File::create(&self.path) {
            let mut writer = BufWriter::new(file);
            for title in titles.iter() {
                writer.write(title.as_bytes()).unwrap();
                writer.write(b"\n").unwrap();
            }
        } else {
            eprintln!("arkbot: unable to create file: '{}'", &self.path);
        }
    }
}
