use std::io::BufRead;

pub fn download(name: &str) -> Box<dyn BufRead> {
    let mut url = "https://dumps.wikimedia.org/".to_owned();
    url.push_str(name);
    url.push_str(".bz2");
    let bz_stream = reqwest::blocking::get(&url);
    let xml_stream = std::io::BufReader::new(bzip2::read::BzDecoder::new(bz_stream.unwrap()));
    Box::new(xml_stream)
}
