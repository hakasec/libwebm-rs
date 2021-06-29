mod ebml;
mod consts;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::ebml::{WebmReader, WebmFile};

    #[test]
    fn test_parser() {
        let file = "./sample/big-buck-bunny_trailer.webm";
        let f = File::open(file).unwrap();
        let document = WebmReader::new(f).parse().unwrap();
        assert_eq!(document.header.get_element().id, 0x1a45dfa3);
        assert_eq!(document.root.get_element().id, 0x18538067);
    }

    #[test]
    fn test_file() {
        let file = "./sample/big-buck-bunny_trailer.webm";
        let f = File::open(file).unwrap();
        let document = WebmFile::open(f);
        assert_eq!(document.header.get_element().id, 0x1a45dfa3);
        assert_eq!(document.root.get_element().id, 0x18538067);
    }
}
