mod ebml;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::ebml;

    #[test]
    fn it_works() {
        let file = "/home/declan/Documents/rat-bday/rats_birthday.webm";
        let f = File::open(file).unwrap();
        println!("{:?}", ebml::EBMLParser::parse(f));
        // let file = "/home/declan/Downloads/SampleVideo_1280x720_10mb.mkv";
        // let f = File::open(file).unwrap();
        // println!("{:?}", ebml::EBMLParser::parse(f));
        assert_eq!(1, 2);
    }

    #[test]
    fn test_parser() {
        let file = "/home/declan/Documents/rat-bday/rats_birthday.webm";
        let f = File::open(file).unwrap();
        let document = ebml::EBMLParser::parse(f);
        assert_eq!(document.header.master.id, 0x1a45dfa3);
        assert_eq!(document.root.master.id, 0x18538067);
    }
}
