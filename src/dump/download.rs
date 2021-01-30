use std::io::BufRead;

pub fn download(url: &str) -> Box<dyn BufRead> {
    let bz_stream = reqwest::blocking::get(url);
    let xml_stream = std::io::BufReader::new(bzip2::read::BzDecoder::new(bz_stream.unwrap()));
    Box::new(xml_stream)
}
