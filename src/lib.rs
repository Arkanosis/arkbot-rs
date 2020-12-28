use quick_xml::events::Event;

enum Tag {
    Title,
    UserName,
    Other,
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
    loop {
        match xml_reader.read_event(&mut buffer) {
            Ok(Event::Empty(ref event)) => {
                match event.name() {
                    b"redirect" => {
                        for attribute in event.attributes() {
                            match attribute {
                                Ok(attribute) => {
                                    if attribute.key == b"title" {
                                        println!("  => {}", std::str::from_utf8(&attribute.unescaped_value().unwrap()).unwrap());
                                        // TODO FIXME
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
                    b"ip" => current_tag = Tag::UserName,
                    b"username" => current_tag = Tag::UserName,
                    _ => current_tag = Tag::Other,
                }
            },
            Ok(Event::End(_)) => current_tag = Tag::Other,
            Ok(Event::Text(ref event)) => {
                match current_tag {
                    Tag::Title => {
                        match event.unescaped() {
                            Ok(ref buffer) => {
                                println!("{}", std::str::from_utf8(buffer).unwrap());
                                // TODO FIXME
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::UserName => {
                        match event.unescaped() {
                            Ok(ref buffer) => {
                                // TODO FIXME
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
}
