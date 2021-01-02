use quick_xml::events::Event;

use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

enum Tag {
    Title,
    Text,
    UserName,
    Other,
}

struct Page {
    title: String,
    text: Option<String>,
    target: Option<String>,
}

trait Process {
    fn process(&mut self, page: &Page);
    fn write_to_file(&mut self);
}

struct Debug {
    // Nothing
}

impl Debug {
    fn new() -> Self {
        Debug {
            // Nothing
        }
    }
}

impl Process for Debug {
    fn process(&mut self, page: &Page) {
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

struct Impasse {
    // TODO
}

impl Impasse {
    fn new() -> Self {
        Impasse {
            // TODO
        }
    }
}

impl Process for Impasse {
    fn process(&mut self, page: &Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}

struct Empty {
    // TODO
}

impl Empty {
    fn new() -> Self {
        Empty {
            // TODO
        }
    }
}

impl Process for Empty {
    fn process(&mut self, page: &Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}

struct LastEdit {
    // TODO
}

impl LastEdit {
    fn new() -> Self {
        LastEdit {
            // TODO
        }
    }
}

impl Process for LastEdit {
    fn process(&mut self, page: &Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}

struct NoPortal {
    // TODO
}

impl NoPortal {
    fn new() -> Self {
        NoPortal {
            // TODO
        }
    }
}

impl Process for NoPortal {
    fn process(&mut self, page: &Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}

struct NoInfobox {
    // TODO
}

impl NoInfobox {
    fn new() -> Self {
        NoInfobox {
            // TODO
        }
    }
}

impl Process for NoInfobox {
    fn process(&mut self, page: &Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}

struct Commercial {
    // TODO
}

impl Commercial {
    fn new() -> Self {
        Commercial {
            // TODO
        }
    }
}

impl Process for Commercial {
    fn process(&mut self, page: &Page) {
        // TODO
    }
    fn write_to_file(&mut self) {
        // TODO
    }
}

struct NamespaceRedirect {
    titles: Vec<String>,
}

impl NamespaceRedirect {
    fn new() -> Self {
        NamespaceRedirect {
            titles: Vec::new(),
        }
    }
}

fn has_namespace(title: &String) -> bool {
    for namespace in [
        "Média:",
        "Spécial:",
        "Discussion:",
        "Utilisateur:",
        "Discussion utilisateur:",
        "Wikipédia:",
        "Discussion Wikipédia:",
        "Fichier:",
        "Discussion fichier:",
        "MediaWiki:",
        "Discussion MediaWiki:",
        "Modèle:",
        "Discussion modèle:",
        "Aide:",
        "Discussion aide:",
        "Catégorie:",
        "Discussion catégorie:",
        "Portail:",
        "Discussion Portail:",
        "Projet:",
        "Discussion Projet:",
        "Référence:",
        "Discussion Référence:",
        "Module:",
        "Discussion module:",
        "Sujet:",
    ].iter() {
        if title.starts_with(namespace) {
            return true;
        }
    }
    return false;
}

impl Process for NamespaceRedirect {
    fn process(&mut self, page: &Page) {
        match page.target {
            Some(ref target) => {
                if !page.title.starts_with("P:") &&
                    has_namespace(target) &&
                    !has_namespace(&page.title) {
                    self.titles.push(page.title.to_string());
                }
            },
            _ => (),
        }
    }
    fn write_to_file(&mut self) {
        self.titles.sort();
        const output_file: &str = "data/frwiki-ns_redirects-latest.txt";
        if let Ok(file) = File::create(output_file) {
            let mut writer = BufWriter::new(file);
            for title in self.titles.iter() {
                writer.write(title.as_bytes());
                writer.write(b"\n");
            }
        } else {
            eprintln!("arkbot: unable to create file: '{}'", output_file);
        }
    }
}

pub fn version() -> &'static str {
    return option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
}

pub fn run() {
    let bz_stream = reqwest::blocking::get("https://dumps.wikimedia.org/frwiki/latest/frwiki-latest-pages-articles.xml.bz2");
    let xml_stream = std::io::BufReader::new(bzip2::read::BzDecoder::new(bz_stream.unwrap()));
    let mut xml_reader = quick_xml::Reader::from_reader(xml_stream);
    let mut buffer = Vec::new();
    let mut current_tag = Tag::Other;
    let mut current_title: Option<String> = None;
    let mut current_text: Option<String> = None;
    let mut current_target: Option<String> = None;
    let mut processors: Vec<Box<dyn Process>> = Vec::new();
    processors.push(Box::new(Debug::new()));
    processors.push(Box::new(Impasse::new()));
    processors.push(Box::new(Empty::new()));
    processors.push(Box::new(LastEdit::new()));
    processors.push(Box::new(NoPortal::new()));
    processors.push(Box::new(NoInfobox::new()));
    processors.push(Box::new(Commercial::new()));
    processors.push(Box::new(NamespaceRedirect::new()));
    loop {
        match xml_reader.read_event(&mut buffer) {
            Ok(Event::Empty(ref event)) => {
                match event.name() {
                    b"redirect" => {
                        for attribute in event.attributes() {
                            match attribute {
                                Ok(attribute) => {
                                    if attribute.key == b"title" {
                                        current_target = Some(std::str::from_utf8(&attribute.unescaped_value().unwrap()).unwrap().to_string());
                                    }
                                }
                                Err(_) => (), // ignore bad attribute in the dump
                            }
                        }
                        current_tag = Tag::Other
                    },
                    _ => current_tag = Tag::Other,
                }
            },
            Ok(Event::Start(ref event)) => {
                match event.name() {
                    b"title" => current_tag = Tag::Title,
                    b"text" => current_tag = Tag::Text,
                    b"ip" => current_tag = Tag::UserName,
                    b"username" => current_tag = Tag::UserName,
                    _ => current_tag = Tag::Other,
                }
            },
            Ok(Event::End(ref event)) => {
                match event.name() {
                    b"title" => current_tag = Tag::Other,
                    b"text" => current_tag = Tag::Other,
                    b"ip" => current_tag = Tag::Other,
                    b"username" => current_tag = Tag::Other,
                    b"page" => {
                        let page = Page {
                            title: current_title.unwrap(),
                            text: current_text,
                            target: current_target,
                        };
                        current_title = None;
                        current_text = None;
                        current_target = None;
                        for processor in processors.iter_mut() {
                            processor.process(&page);
                        }
                        current_tag = Tag::Other;
                    },
                    _ => current_tag = Tag::Other,
                }
            }
            Ok(Event::Text(ref event)) => {
                match current_tag {
                    Tag::Title => {
                        match event.unescaped() {
                            Ok(ref buffer) => {
                                current_title = Some(std::str::from_utf8(buffer).unwrap().to_string());
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::Text => {
                        match event.unescaped() {
                            Ok(ref buffer) => {
                                current_text = Some(std::str::from_utf8(buffer).unwrap().to_string());
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::UserName => {
                        match event.unescaped() {
                            Ok(ref buffer) => {
                                // TODO
                            },
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::Other => (),
                }
            },
            Err(error) => {
                eprintln!("arkbot: XML parsing error at position {}: {:?}", xml_reader.buffer_position(), error);
                break;
            },
            Ok(Event::Eof) => break,
            _ => (),
        }
        buffer.clear();
    }
    for processor in processors.iter_mut() {
        processor.write_to_file();
    }
}
