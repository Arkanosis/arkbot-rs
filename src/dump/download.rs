use reqwest::blocking::Client;
use std::io::BufRead;

pub fn download(url: &str) -> Box<dyn BufRead> {
    let client = Client::builder()
        .user_agent(crate::user_agent())
        .build()
        .unwrap();
    let bz_stream = client
        .get(url)
        .send();
    let xml_stream = std::io::BufReader::new(bzip2::read::BzDecoder::new(bz_stream.unwrap()));
    Box::new(xml_stream)
 }
