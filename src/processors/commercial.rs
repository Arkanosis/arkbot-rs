use crate::processors;
use crate::wiki;

use regex::Regex;

pub struct Commercial {
    titles: Vec<String>,
    ignore: Regex,
}

impl Commercial {
    pub fn new() -> Self {
        Self {
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
    fn finalize(&mut self) {
        self.titles.sort();
    }
    fn lines(&self) -> &Vec<String> {
        &self.titles
    }
}
