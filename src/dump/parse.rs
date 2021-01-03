use crate::wiki;

use quick_xml::events::Event;

use std::io::BufRead;

enum Tag {
    Namespace,
    Other,
    Text,
    Title,
    UserName,
}

pub fn parse<Callback: FnMut(&wiki::Page) -> ()>(stream: &mut dyn BufRead, mut callback: Callback) {
    let mut xml_reader = quick_xml::Reader::from_reader(stream);
    let mut buffer = Vec::new();
    let mut current_tag = Tag::Other;
    let mut current_namespace = 0;
    let mut current_title: Option<String> = None;
    let mut current_text: Option<String> = None;
    let mut current_target: Option<String> = None;
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
                    b"ip" => current_tag = Tag::UserName,
                    b"ns" => current_tag = Tag::Namespace,
                    b"text" => current_tag = Tag::Text,
                    b"title" => current_tag = Tag::Title,
                    b"username" => current_tag = Tag::UserName,
                    _ => current_tag = Tag::Other,
                }
            },
            Ok(Event::End(ref event)) => {
                match event.name() {
                    b"page" => {
                        let page = wiki::Page {
                            namespace: current_namespace,
                            title: current_title.unwrap(),
                            text: current_text,
                            target: current_target,
                        };
                        current_namespace = 0;
                        current_title = None;
                        current_text = None;
                        current_target = None;
                        callback(&page);
                        current_tag = Tag::Other;
                    },
                    _ => current_tag = Tag::Other,
                }
            }
            Ok(Event::Text(ref event)) => {
                match current_tag {
                    Tag::Namespace => {
                        match event.unescaped() {
                            Ok(ref buffer) => {
                                current_namespace = std::str::from_utf8(buffer).unwrap().parse().unwrap();
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
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
}
