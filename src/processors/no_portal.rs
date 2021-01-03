use crate::processors;
use crate::wiki;

pub struct NoPortal {
    // TODO
}

impl NoPortal {
    pub fn new() -> Self {
        NoPortal {
            // TODO
        }
    }
}

impl processors::Process for NoPortal {
    fn process(&mut self, page: &wiki::Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}
