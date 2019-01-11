mod ebml;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::ebml;

    #[test]
    fn test_parser() {
        let file = "./sample/big-buck-bunny_trailer.webm";
        let f = File::open(file).unwrap();
        let document = ebml::WebmReader::new(f).parse().unwrap();
        assert_eq!(document.header.get_element().id, 0x1a45dfa3);
        assert_eq!(document.root.get_element().id, 0x18538067);
    }

    #[test]
    fn test_file() {
        let file = "./sample/big-buck-bunny_trailer.webm";
        let f = File::open(file).unwrap();
        let document = ebml::WebmFile::open(f);
        assert_eq!(document.header.get_element().id, 0x1a45dfa3);
        assert_eq!(document.root.get_element().id, 0x18538067);
    }
}
