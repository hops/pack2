use std::path::PathBuf;

use bstr::{io::BufReadExt, ByteSlice};
use pack2_util::*;

pub fn unhex(input: Option<PathBuf>, output: Option<PathBuf>) {
    let reader = get_reader(input);
    let mut writer = get_writer(output);

    for result in reader.byte_lines() {
        let (mut line, _line_len) = decode_hex_if_needed(result.unwrap());
        line.push(b'\n');
        mywrite(&mut line.as_bytes(), &mut writer);
    }
}
