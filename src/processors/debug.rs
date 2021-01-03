use crate::processors;
use crate::wiki;

pub struct Debug {
    // Nothing
}

impl Debug {
    pub fn new() -> Self {
        Debug {
            // Nothing
        }
    }
}

impl processors::Process for Debug {
    fn process(&mut self, page: &wiki::Page) {
        match page.target {
            None => {
                if let Some(ref text) = page.text {
                    println!("Page: '{}', length: {}", page.title, text.len());
                } else {
                    eprintln!("arkbot: page without text: '{}'", page.title);
                }
            },
            Some(ref target) => {
                println!("Page: '{}', redirect to: {}", page.title, target);
            },
        }
    }
    fn write_to_file(&mut self) {
        // Nothing
    }
}
