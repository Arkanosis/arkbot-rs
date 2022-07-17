pub mod commercial;
pub mod debug;
pub mod empty;
pub mod impasse;
pub mod last_edit;
pub mod namespace_redirect;
pub mod no_infobox;
pub mod no_portal;

use crate::wiki;

pub trait Process {
    fn process(&mut self, page: &wiki::Page);
    fn finalize(&mut self);
    fn lines(&self) -> &Vec<String>;
}

pub use commercial::Commercial;
pub use debug::Debug;
pub use empty::Empty;
pub use impasse::Impasse;
pub use last_edit::LastEdit;
pub use namespace_redirect::NamespaceRedirect;
pub use no_infobox::NoInfobox;
pub use no_portal::NoPortal;
