use crate::wiki;

use quick_xml::events::Event;

use std::io::BufRead;

enum Tag {
    Namespace,
    Other,
    Text,
    Timestamp,
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
    let mut current_username: Option<String> = None;
    let mut current_timestamp: Option<String> = None;
    loop {
        match xml_reader.read_event_into(&mut buffer) {
            Ok(Event::Empty(ref event)) => {
                match event.name().as_ref() {
                    b"redirect" => {
                        for attribute in event.attributes() {
                            match attribute {
                                Ok(attribute) => {
                                    if attribute.key.as_ref() == b"title" {
                                        let escaped_value = attribute.unescape_value();
                                        current_target = Some(std::str::from_utf8(&escaped_value.unwrap().as_bytes()).unwrap().to_string());
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
                match event.name().as_ref() {
                    b"ip" => current_tag = Tag::UserName,
                    b"ns" => current_tag = Tag::Namespace,
                    b"text" => current_tag = Tag::Text,
                    b"timestamp" => current_tag = Tag::Timestamp,
                    b"title" => current_tag = Tag::Title,
                    b"username" => current_tag = Tag::UserName,
                    _ => current_tag = Tag::Other,
                }
            },
            Ok(Event::End(ref event)) => {
                match event.name().as_ref() {
                    b"page" => {
                        let page = wiki::Page {
                            namespace: current_namespace,
                            title: current_title.unwrap(),
                            text: current_text,
                            target: current_target,
                            username: current_username,
                            timestamp: current_timestamp,
                        };
                        current_namespace = 0;
                        current_title = None;
                        current_text = None;
                        current_target = None;
                        current_username = None;
                        current_timestamp = None;
                        callback(&page);
                        current_tag = Tag::Other;
                    },
                    _ => current_tag = Tag::Other,
                }
            }
            Ok(Event::Text(ref event)) => {
                let escaped_event = event.decode();
                match current_tag {
                    Tag::Namespace => {
                        match escaped_event {
                            Ok(ref buffer) => {
                                current_namespace = std::str::from_utf8(buffer.as_bytes()).unwrap().parse().unwrap();
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::Text => {
                        match escaped_event {
                            Ok(ref buffer) => {
                                current_text = Some(std::str::from_utf8(buffer.as_bytes()).unwrap().to_string());
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::Timestamp => {
                        match escaped_event {
                            Ok(ref buffer) => {
                                current_timestamp = Some(std::str::from_utf8(buffer.as_bytes()).unwrap().to_string());
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::Title => {
                        match escaped_event {
                            Ok(ref buffer) => {
                                current_title = Some(std::str::from_utf8(buffer.as_bytes()).unwrap().to_string());
                            }
                            Err(_) => (), // ignore encoding error in the dump
                        }
                    },
                    Tag::UserName => {
                        match escaped_event {
                            Ok(ref buffer) => {
                                current_username = Some(std::str::from_utf8(buffer.as_bytes()).unwrap().to_string());
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
