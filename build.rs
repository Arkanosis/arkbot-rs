use vergen_git2::{
    Emitter,
    Git2Builder,
};

fn main() {
    let git2 = Git2Builder::default()
        .sha(true)
        .build()
        .unwrap();
    Emitter::default()
        .add_instructions(&git2)
        .unwrap()
        .emit()
        .unwrap();
}
