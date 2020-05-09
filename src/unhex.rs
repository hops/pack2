use std::io::{self};
use std::path::PathBuf;

use bstr::{ByteSlice, io::BufReadExt};
use pack2_util::*;

pub fn unhex(input: Option<PathBuf>, output: Option<PathBuf>) {

    let reader     = get_reader(input);
    let mut writer = get_writer(output);

    for result in reader.byte_lines() {
        let (mut line, _line_len) = decode_hex_if_needed(result.unwrap());
        line.push('\n' as u8);
        io::copy(&mut line.as_bytes(), &mut writer).unwrap();
    }
}
