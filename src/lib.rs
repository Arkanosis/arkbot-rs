mod dump;
mod processors;
mod wiki;

pub fn version() -> &'static str {
    return option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
}

pub fn run() {
    let mut processors: Vec<Box<dyn processors::Process>> = Vec::new();
    processors.push(Box::new(processors::Commercial::new()));
    //processors.push(Box::new(processors::Debug::new()));
    processors.push(Box::new(processors::Empty::new()));
    //processors.push(Box::new(processors::Impasse::new()));
    processors.push(Box::new(processors::LastEdit::new()));
    processors.push(Box::new(processors::NamespaceRedirect::new()));
    processors.push(Box::new(processors::NoInfobox::new()));
    processors.push(Box::new(processors::NoPortal::new()));

    dump::monitor("frwiki", "pages-articles.xml.bz2", |date, url| {
        println!("Processing dump for {}", &date);
        let mut dump_stream = dump::download(url);
        dump::parse(dump_stream.as_mut(), |page| {
            for processor in processors.iter_mut() {
                processor.process(&page);
            }
        });

        for processor in processors.iter_mut() {
            processor.write_to_file();
        }
    });
}
