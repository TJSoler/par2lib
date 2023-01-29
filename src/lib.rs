use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use binread::{BinRead, io::Cursor};

#[derive(BinRead, Debug)]
#[br(little, magic = b"PAR2\0PKT")]
struct Par2Packet {
    length: u64,
   
    #[br(count = 16)]
    hash: Vec<u8>,
    
    #[br(count = 16)]
    set_id: Vec<u8>,
   
    #[br(count = 16)]
    packet_type: Vec<u8>,
}

pub fn parse(path: &Path) -> String { 
    use std::str;
    let f = File::open(path).unwrap();
    let mut reader = BufReader::new(f);

    let mut packet = Par2Packet::read(&mut reader).unwrap();

    let s = match str::from_utf8(&packet.packet_type) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let t = s.replace("\0", " ");

    return t.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let main_packet = parse(&Path::new("./testdata/test.par2"));
        assert_eq!(main_packet, "PAR 2.0 Main");

        let recv_slice_packet = parse(&Path::new("./testdata/test.vol000+01.par2"));
        assert_eq!(recv_slice_packet, "PAR 2.0 RecvSlic");
    }
}
