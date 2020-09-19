use std::path::PathBuf;

use bstr::{io::BufReadExt, ByteSlice};
use pack2_util::*;

// TODO: This is a very basic mask "parser" we should make sure it's a valid mask.
// We don't support ?h, ?H or custom masks yet.
fn parse_mask(mask: String) -> (Vec<u8>, usize) {
    let mask_len = mask.len() / 2;
    let mut ret: Vec<u8> = Vec::new();
    for c in mask.chars() {
        match c {
            'l' => ret.push(1),
            'u' => ret.push(2),
            'd' => ret.push(4),
            's' => ret.push(8),
            'a' => ret.push(15),
            'b' => ret.push(31),
            _ => (),
        }
    }

    (ret, mask_len)
}

pub fn filtermask(input: Option<PathBuf>, output: Option<PathBuf>, mask: String) {
    let reader = get_reader(input);
    let mut writer = get_writer(output);
    let (filter, mask_len) = parse_mask(mask);

    let mut processed = 0;
    let mut skipped = 0;

    for result in reader.byte_lines() {
        let (mut line, line_len) = decode_hex_if_needed(result.unwrap());

        if line_len != mask_len {
            skipped += 1;
            continue;
        }

        let mut matched = true;
        let mut idx = 0;
        for byte in line.iter() {
            if filter[idx] & CHAR2BITMAP[*byte as usize] == 0 {
                matched = false;
                break;
            }
            idx += 1;
        }

        if matched {
            processed += 1;
            line.push('\n' as u8);
            mywrite(&mut line.as_bytes(), &mut writer);
        }
    }
    let total_lines = processed + skipped;
    eprintln!(
        "wrote {} out of {} lines. Skipped: {}",
        processed, total_lines, skipped
    );
}
