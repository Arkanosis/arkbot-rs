use serde_derive::Deserialize;

const USAGE: &str = "
Usage: arkbot
       arkbot test
       arkbot -h | --help
       arkbot --version

Options:
    -h, --help               Show this screen.
    --version                Show version.
";

#[derive(Deserialize)]
struct Args {
    cmd_test: bool,
    flag_version: bool,
}

fn main() {
    let args: Args =
        docopt::Docopt::new(USAGE)
            .and_then(|docopts|
                docopts.argv(std::env::args().into_iter())
                   .deserialize()
            )
            .unwrap_or_else(|error|
                error.exit()
            );

    if args.flag_version {
        println!("arkbot v{}", arkbot::version());
    } else if args.cmd_test {
        arkbot::test();
    } else {
        arkbot::run();
    }
}
