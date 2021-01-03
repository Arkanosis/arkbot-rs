use crate::processors;
use crate::wiki;

pub struct NoInfobox {
    // TODO
}

impl NoInfobox {
    pub fn new() -> Self {
        NoInfobox {
            // TODO
        }
    }
}

impl processors::Process for NoInfobox {
    fn process(&mut self, page: &wiki::Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}
