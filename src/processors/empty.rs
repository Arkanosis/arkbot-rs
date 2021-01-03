use crate::processors;
use crate::wiki;

use regex::Regex;

use std::{
    collections::HashSet,
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

pub struct Empty {
    titles: Vec<String>,
    empty: Regex,
    ignore: HashSet<String>,
}

impl Empty {
    pub fn new() -> Self {
        let mut result = Empty {
            titles: Vec::new(),
            empty: Regex::new(r"^[[:space:]]*$").unwrap(),
            ignore: HashSet::new(),
        };
        result.ignore.insert("MediaWiki:Anonnotice".to_string());
        result.ignore.insert("MediaWiki:Excontentauthor".to_string());
        result.ignore.insert("MediaWiki:Reader-demographics-1-description".to_string());
        result.ignore.insert("MediaWiki:Sitenotice".to_string());
        result
    }
}

impl processors::Process for Empty {
    fn process(&mut self, page: &wiki::Page) {
        if page.target == None {
            if !self.ignore.contains(&page.title) {
                if let Some(text) = &page.text {
                    if self.empty.is_match(&text) {
                        self.titles.push(page.title.to_string());
                    }
                } else {
                    self.titles.push(page.title.to_string());
                }
            }
        }
    }
    fn write_to_file(&mut self) {
        self.titles.sort();
        const output_file: &str = "data/frwiki-empty_pages-latest.txt";
        if let Ok(file) = File::create(output_file) {
            let mut writer = BufWriter::new(file);
            for title in self.titles.iter() {
                writer.write(title.as_bytes()).unwrap();
                writer.write(b"\n").unwrap();
            }
        } else {
            eprintln!("arkbot: unable to create file: '{}'", output_file);
        }
    }
}
