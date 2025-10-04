use quick_xml::events::Event;

use std::fmt::Write;

pub fn monitor<Callback: FnMut(&str, &str) -> ()>(wiki: &str, dump: &str, mut callback: Callback) {
    let url = format!("https://dumps.wikimedia.org/{}/latest/{}-latest-{}-rss.xml", wiki, wiki, dump);

    // TODO loop here and only call back when the date has changed
    // TODO wait for X time (additional parameter?) before retrying
    // TODO get last known date from cache file
    let stream = reqwest::blocking::get(&url);
    let xml_stream = std::io::BufReader::new(stream.unwrap());
    let mut xml_reader = quick_xml::Reader::from_reader(xml_stream);
    let mut buffer = Vec::new();
    let mut on_link = false;
    loop {
        match xml_reader.read_event_into(&mut buffer) {
            Ok(Event::Start(ref event)) => {
                if event.name().as_ref() == b"link" {
                    on_link = true;
                }
            },
            Ok(Event::Text(ref event)) => {
                if on_link {
                    let escaped_event = event.decode();
                    match escaped_event {
                        Ok(ref buffer) => {
                            let mut url = std::str::from_utf8(buffer.as_bytes()).unwrap().to_string();
                            let date = url.rsplitn(2, "/").next().unwrap().to_string();
                            write!(&mut url, "/{}-{}-{}", wiki, &date, dump).unwrap();
                            callback(&date, &url);
                        }
                        Err(_) => (), // ignore encoding error in the dump
                    }
                    break;
                }
            },
            Err(error) => {
                eprintln!("arkbot: RSS parsing error at position {}: {:?}", xml_reader.buffer_position(), error);
                break;
            },
            Ok(Event::Eof) => break,
            _ => (),
        }
    }
}
