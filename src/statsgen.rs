use std::iter::FromIterator;
use std::path::PathBuf;

use bstr::{io::BufReadExt, ByteSlice};
use hashbrown::HashMap;
use pack2_util::*;

const DEFAULT_ENCODED: u64 = 0x00000000ffff0000;

#[inline(always)]
fn encode_count_min_max(count: u32, min: u16, max: u16) -> u64 {
    let encoded: u64 = (count as u64) << 32 | (min as u64) << 16 | (max as u64) << 0;
    encoded
}

#[inline(always)]
fn decode_count_min_max(encoded: u64) -> (u32, u16, u16) {
    let count: u32 = (encoded >> 32) as u32;
    let min: u16 = ((encoded >> 16) & 0xffff) as u16;
    let max: u16 = ((encoded >> 0) & 0xffff) as u16;
    (count, min, max)
}

pub fn gen(
    input: Option<PathBuf>,
    output: Option<PathBuf>,
    separator: Option<char>,
    min_length: u16,
    max_length: u16,
) {
    let bitmap2string = get_bitmap2string();
    let reader = get_reader(input);

    let mut masks: HashMap<Vec<u8>, u32> = HashMap::new();
    let mut simple_masks: HashMap<Vec<u8>, u64> = HashMap::new();
    let mut length: HashMap<u16, u32> = HashMap::new();
    let mut charsets: HashMap<u8, u64> = HashMap::new();

    let mut processed_lines: usize = 0;
    let mut skipped_lined: usize = 0;
    let mut min_len: usize = usize::MAX;
    let mut max_len: usize = 0;

    let mut mask: Vec<u8> = Vec::new();
    let mut simple_mask: Vec<u8> = Vec::new();

    for result in reader.byte_lines() {
        let (line, line_len) = decode_hex_if_needed(result.unwrap());

        if line_len < min_length.into() || line_len > max_length.into() {
            skipped_lined += 1;
            continue;
        }

        if min_len > line_len {
            min_len = line_len
        }
        if line_len > max_len {
            max_len = line_len
        }

        let mut last: u8 = 0;

        let mut charset: u8 = 0;
        let mut skip = false;

        for byte in line.iter() {
            mask.push(CHAR2MASK[*byte as usize]);
            charset |= CHAR2BITMAP[*byte as usize];
            let char_mapped = CHAR2SMASK[*byte as usize];
            if last == 0 || (last != char_mapped && !skip) {
                simple_mask.push(char_mapped);
                last = char_mapped;
            }
            if simple_mask.len() > 4 {
                skip = true;
                simple_mask.clear();
                simple_mask.push(255);
            }
        }

        *masks.entry((&*mask).to_vec()).or_insert(0) += 1;
        *length.entry(line_len as u16).or_insert(0) += 1;
        let charsets_entry = charsets.entry(charset).or_insert(DEFAULT_ENCODED);
        let (mut charsets_count, mut charsets_min_len, mut charsets_max_len) =
            decode_count_min_max(*charsets_entry);

        charsets_count += 1;
        if charsets_min_len as usize > line_len {
            charsets_min_len = line_len as u16
        }
        if line_len > charsets_max_len as usize {
            charsets_max_len = line_len as u16
        }

        *charsets_entry = encode_count_min_max(charsets_count, charsets_min_len, charsets_max_len);

        let simple_mask_entry = simple_masks
            .entry((&*simple_mask).to_vec())
            .or_insert(DEFAULT_ENCODED);
        let (mut simple_count, mut simple_min_len, mut simple_max_len) =
            decode_count_min_max(*simple_mask_entry);

        simple_count += 1;
        if simple_min_len as usize > line_len {
            simple_min_len = line_len as u16
        }
        if line_len > simple_max_len as usize {
            simple_max_len = line_len as u16
        }

        *simple_mask_entry = encode_count_min_max(simple_count, simple_min_len, simple_max_len);

        mask.clear();
        simple_mask.clear();
        processed_lines += 1;
    }

    let total_lines = processed_lines + skipped_lined;
    eprintln!(
        "[+] Analyzed {} / {} passwords.",
        processed_lines, total_lines
    );

    let mut freq_len = Vec::from_iter(length);
    freq_len.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!(
        "[*] Length distribution: (min: {} max: {})",
        min_len, max_len
    );
    for (len, count) in freq_len {
        let percent = 100.0 / processed_lines as f64 * count as f64;
        eprintln!("[+] {: >26}: {: >6.2}% ({})", len, percent, count);
    }

    let mut freq_charsets = Vec::from_iter(charsets);
    freq_charsets.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!("\n[*] Charset distribution:                    count   min   max");
    for (charset, encoded) in freq_charsets {
        let (count, min_len, max_len) = decode_count_min_max(encoded);
        let percent = 100.0 / processed_lines as f64 * count as f64;
        eprintln!(
            "[+] {: >26}: {: >6.2}% {: >10} {: >5} {: >5}",
            bitmap2string[charset as usize], percent, count, min_len, max_len
        );
    }

    let mut freq_simple_masks = Vec::from_iter(simple_masks);
    freq_simple_masks.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!("\n[*] Simple masks distribution:               count   min   max");
    for (simple_mask, encoded) in freq_simple_masks {
        let (count, min_len, max_len) = decode_count_min_max(encoded);
        let percent = 100.0 / processed_lines as f64 * count as f64;
        let mut print_simple_mask: Vec<&str> = Vec::new();
        for mapped_mask in simple_mask {
            match mapped_mask {
                1 => print_simple_mask.push("string"),
                4 => print_simple_mask.push("digit"),
                8 => print_simple_mask.push("special"),
                16 => print_simple_mask.push("binary"),
                255 => print_simple_mask.push("othermask"),
                _ => (),
            }
        }

        eprintln!(
            "[+] {: >26}: {: >6.2}% {: >10} {: >5} {: >5}",
            print_simple_mask.join(""),
            percent,
            count,
            min_len,
            max_len
        );
    }
    let mut freq_masks = Vec::from_iter(masks);
    freq_masks.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!("\n[*] Masks (top 25):");
    let mut top = 0;
    let mut print_mask = vec!['?' as u8; max_len * 2];

    let mut writer = get_writer(output);

    let separator: char = match separator {
        None => '\t',
        Some(sep) => sep,
    };

    for (mask, count) in freq_masks {
        let mut idx = 0;
        for c in mask {
            print_mask[idx * 2 + 1] = c;
            idx += 1;
        }
        let out_mask = &print_mask[0..idx * 2].to_str().unwrap();
        let percent = 100.0 / processed_lines as f64 * count as f64;
        if top < 25 {
            eprintln!("[+] {: >26}: {: >6.2}% ({})", out_mask, percent, count);
            top += 1;
        }
        let out = &*format!(
            "{}{}{:.4}{}{}\n",
            out_mask, separator, percent, separator, count
        );
        mywrite(&mut out.as_bytes(), &mut writer);
    }
}
