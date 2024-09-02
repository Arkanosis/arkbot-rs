use crate::processors;
use crate::wiki;

pub struct Debug {
    titles: Vec<String>,
}

impl Debug {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
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
    fn finalize(&mut self) {
        // Nothing
    }
    fn lines(&self) -> &Vec<String> {
        &self.titles
    }
}
