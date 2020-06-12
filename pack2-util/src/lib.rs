use std::io::{self, BufRead, BufReader, Write, BufWriter};
use std::fs::File;
use std::path::PathBuf;

use faster_hex::{hex_decode, hex_encode};

#[inline(always)]
pub fn contains_nonprintable(line: &Vec<u8>) -> bool {
    for c in line {
        if (*c as i8).wrapping_add(1) < 0x21 { return true }
    }
    false
}

pub fn encode_hex_if_needed(line: Vec<u8>, out: &mut Vec<u8>) {
    if contains_nonprintable(&line) {
        let mut hex_encoded = vec![0u8; line.len() * 2];
        hex_encode(&line, &mut hex_encoded).unwrap();

        // clear output buffer
        out.clear();
        out.extend_from_slice("$HEX[".as_bytes());
        out.extend_from_slice(&hex_encoded.as_slice());
        out.push(']' as u8);
    } else {
    *out = line
    }
}

pub fn decode_hex_if_needed(mut line: Vec<u8>) -> (Vec<u8>, usize) {
    let mut line_len = line.len();
    if line.starts_with("$HEX[".as_bytes()) && line.ends_with("]".as_bytes()) {
        line_len = (line_len - 6) / 2;
        let mut hex_decoded = vec![0; line_len];
        let res = hex_decode(&line[5..line.len() - 1], &mut hex_decoded);
        match res {
            Ok(_)  => line = hex_decoded,
            // not valid $HEX encoding, treat as "normal" password
            Err(_) => line_len = line.len(),
        }
    }
    (line, line_len)
}

pub fn get_reader(input: Option<PathBuf>) -> Box<dyn BufRead> {
    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap()))
    };
    reader
}

pub fn get_writer(output: Option<PathBuf>) -> Box<dyn Write> {
    let writer: Box<dyn Write> = match output {
        None => Box::new(BufWriter::new(io::stdout())),
        Some(filename) => Box::new(BufWriter::new(File::create(filename).unwrap()))
    };
    writer
}

pub fn get_bitmap2string() -> Vec<&'static str> {
    let bitmap2string: Vec<&str> = vec![
        "invalid",
        "loweralpha",
        "upperalpha",
        "mixedalpha",
        "numeric",
        "loweralphanum",
        "upperalphanum",
        "mixedalphanum",
        "special",
        "loweralphaspecial",
        "upperalphaspecial",
        "mixedalphaspecial",
        "specialnum",
        "loweralphaspecialnum",
        "upperalphaspecialnum",
        "mixedalphaspecialnum",
        "binary",
        "loweralphabin",
        "upperalphabin",
        "mixedalphabin",
        "numericbin",
        "loweralphanumbin",
        "upperalphanumbin",
        "mixedalphanumbin",
        "specialbin",
        "loweralphaspecialbin",
        "upperalphaspecialbin",
        "mixedalphaspecialbin",
        "specialnumbin",
        "loweralphaspecialnumbin",
        "upperalphaspecialnumbin",
        "mixedalphaspecialnumbin",
    ];
    bitmap2string
}

pub const CHAR2MASK: [u8; 256] = [ 
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x73, 0x73, 0x73, 0x73, 0x73, 0x73, 0x73, 0x73,
    0x73, 0x73, 0x73, 0x73, 0x73, 0x73, 0x73, 0x73,
    0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64, 0x64,
    0x64, 0x64, 0x73, 0x73, 0x73, 0x73, 0x73, 0x73,
    0x73, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
    0x75, 0x75, 0x75, 0x73, 0x73, 0x73, 0x73, 0x73,
    0x73, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c,
    0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c,
    0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c, 0x6c,
    0x6c, 0x6c, 0x6c, 0x73, 0x73, 0x73, 0x73, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62, 0x62,
    ];  

pub const CHAR2BITMAP: [u8; 256] = [ 
    16, 16, 16, 16, 16, 16, 16, 16, 
    16, 16, 16, 16, 16, 16, 16, 16, 
    16, 16, 16, 16, 16, 16, 16, 16, 
    16, 16, 16, 16, 16, 16, 16, 16, 
     8,  8,  8,  8,  8,  8,  8,  8,  
     8,  8,  8,  8,  8,  8,  8,  8,  
     4,  4,  4,  4,  4,  4,  4,  4,
     4,  4,  8,  8,  8,  8,  8,  8,
     8,  2,  2,  2,  2,  2,  2,  2,
     2,  2,  2,  2,  2,  2,  2,  2,
     2,  2,  2,  2,  2,  2,  2,  2,
     2,  2,  2,  8,  8,  8,  8,  8,
     8,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  8,  8,  8,  8, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
];

pub const CHAR2SMASK: [u8; 256] = [
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
     8,  8,  8,  8,  8,  8,  8,  8,
     8,  8,  8,  8,  8,  8,  8,  8,
     4,  4,  4,  4,  4,  4,  4,  4,
     4,  4,  8,  8,  8,  8,  8,  8,
     8,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  8,  8,  8,  8,  8,
     8,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  1,  1,  1,  1,  1,
     1,  1,  1,  8,  8,  8,  8, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
    16, 16, 16, 16, 16, 16, 16, 16,
];

