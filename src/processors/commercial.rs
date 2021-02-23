use crate::processors;
use crate::wiki;

use regex::Regex;

use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

pub struct Commercial {
    titles: Vec<String>,
    ignore: Regex,
}

impl Commercial {
    pub fn new() -> Self {
        Commercial {
            titles: Vec::new(),
            ignore: Regex::new(r"(?imsx)
                # references
                <ref[^>]*/\s*>|
                <ref[^>]*>.*?</ref>|

                # images in templates
                \|\s*image\s*=\s*[^\|$\}]+|

                # multimedia files
                \[\[(?:file|fichier|image)\s*:\s*[^\]]+|

                # legitimate use in names
                è®e
            ").unwrap(),
        }
    }
}

impl processors::Process for Commercial {
    fn process(&mut self, page: &wiki::Page) {
        if page.namespace == 0 {
            if page.target == None {
                if let Some(text) = &page.text {
                    let text = self.ignore.replace_all(&text, "");
                    for c in "Ⓡ®℗™℠".chars() {
                        if text.contains(c) {
                            self.titles.push(page.title.to_string());
                            return;
                        }
                    }
                }
            }
        }
    }
    fn write_to_file(&mut self, output_directory: &str) {
        self.titles.sort();
        let output_file = format!("{}/frwiki-commercials-latest.txt", output_directory);
        if let Ok(file) = File::create(&output_file) {
            let mut writer = BufWriter::new(file);
            for title in self.titles.iter() {
                writer.write(title.as_bytes()).unwrap();
                writer.write(b"\n").unwrap();
            }
        } else {
            eprintln!("arkbot: unable to create file: '{}'", &output_file);
        }
    }
}
