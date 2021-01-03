use crate::processors;
use crate::wiki;

pub struct LastEdit {
    // TODO
}

impl LastEdit {
    pub fn new() -> Self {
        LastEdit {
            // TODO
        }
    }
}

impl processors::Process for LastEdit {
    fn process(&mut self, page: &wiki::Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}
