pub mod debug;
pub mod file;
pub mod wiki;

pub trait Publish {
    fn publish(&self, titles: &Vec<String>);
}

pub use debug::Debug;
pub use file::File;
pub use wiki::Wiki;
