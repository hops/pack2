use std::io::{self, BufRead, BufReader, Write, BufWriter};
use std::iter::FromIterator;
use std::fs::File;
use std::path::PathBuf;

use bstr::{ByteSlice, io::BufReadExt};
use hashbrown::HashMap;
use faster_hex::hex_decode;

mod common;

pub fn gen(input: Option<PathBuf>, output: Option<PathBuf>, separator: Option<char>) {

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

    let reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(File::open(filename).unwrap()))
    };

    let mut masks:        HashMap<Vec<u8>, u32> = HashMap::new();
    let mut simple_masks: HashMap<Vec<u8>, u32> = HashMap::new();
    let mut length:       HashMap<u16, u32> = HashMap::new();
    let mut charsets:     HashMap< u8, u32> = HashMap::new();

    let mut total_lines: usize = 0;
    let mut skipped:     usize = 0;
    let mut min_len:     usize = usize::MAX;
    let mut max_len:     usize = 0;


    let mut mask: Vec<u8> = Vec::new();
    let mut simple_mask: Vec<u8> = Vec::new();

    for result in reader.byte_lines() {
        let mut line = result.unwrap();
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

        if line_len == 0 || line_len > usize::MAX {
            skipped += 1;
            continue;
        }

        if min_len > line_len { min_len = line_len }
        if line_len > max_len { max_len = line_len }

        let mut last: u8 = 0;

        let mut charset: u8 = 0;
        let mut skip = false;

        for byte in line.iter() {
            mask.push(common::CHAR2MASK[*byte as usize]);
            charset |= common::CHAR2BITMAP[*byte as usize];
            let char_mapped = common::CHAR2SMASK[*byte as usize];
            if last == 0 || (last != char_mapped && !skip) {
                simple_mask.push(char_mapped);
                last = char_mapped;
            }
            if simple_mask.len() > 4 {
                skip = true;
            }
        }

        *masks.entry((&*mask).to_vec()).or_insert(0) += 1;
        *length.entry(line_len as u16).or_insert(0) += 1;
        *charsets.entry(charset).or_insert(0) += 1;
        if skip  {
            *simple_masks.entry(vec![255]).or_insert(0) += 1;
        } else {
            *simple_masks.entry((&*simple_mask).to_vec()).or_insert(0) += 1;
        }

        mask.clear();
        simple_mask.clear();
        total_lines += 1;
    }

    let num_lines = total_lines - skipped;
    eprintln!("[+] Analyzed {} / {} passwords.", num_lines, total_lines);

    let mut freq_len = Vec::from_iter(length);
    freq_len.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!("[*] Length distribution: (min: {} max: {})", min_len, max_len);
    for (len, count) in freq_len {
        let percent = 100.0 / total_lines as f64 * count as f64;
        eprintln!("[+] {: >26}: {: >5.2}% ({})", len, percent, count);
    }

    let mut freq_charsets = Vec::from_iter(charsets);
    freq_charsets.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!("\n[*] Charset distribution:");
    for (charset, count) in freq_charsets {
        let percent = 100.0 / total_lines as f64 * count as f64;
        eprintln!("[+] {: >26}: {: >5.2}% ({})", bitmap2string[charset as usize], percent, count);
    }

    let mut freq_simple_masks = Vec::from_iter(simple_masks);
    freq_simple_masks.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!("\n[*] Simple masks distribution:");
    for (simple_mask, count) in freq_simple_masks {
        let percent = 100.0 / total_lines as f64 * count as f64;
        let mut print_simple_mask: Vec<&str> = Vec::new();
        for mapped_mask in simple_mask {
            match mapped_mask {
                 1 => print_simple_mask.push("string"),
                 4 => print_simple_mask.push("digit"),
                 8 => print_simple_mask.push("special"),
                16 => print_simple_mask.push("binary"),
               255 => print_simple_mask.push("othermask"),
                 _ => ()
            }
        }

        eprintln!("[+] {: >26}: {: >5.2}% ({})", print_simple_mask.join(""), percent, count);
    }
    let mut freq_masks = Vec::from_iter(masks);
    freq_masks.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    eprintln!("\n[*] Masks (top 25):");
    let mut top = 0;
    let mut print_mask = vec!['?' as u8; max_len * 2];

    let mut writer: Box<dyn Write> = match output {
        None => Box::new(BufWriter::new(io::stdout())),
        Some(filename) => Box::new(BufWriter::new(File::create(filename).unwrap()))
    };

    let separator: char = match separator {
        None => '\t',
        Some(sep) => sep
    };

    for (mask, count) in freq_masks {
        let mut idx = 0;
        for c in mask {
            print_mask[idx * 2 + 1] = c;
            idx += 1;
        }
        let out_mask = &print_mask[0..idx*2].to_str().unwrap();
        if top < 25 {
            let percent = 100.0 / total_lines as f64 * count as f64;
            eprintln!("[+] {: >26}: {: >5.2}% ({})", out_mask, percent, count);
            top += 1;
        }
        let out = &*format!("{}{}{}\n", out_mask, separator, count);
        io::copy(&mut out.as_bytes(), &mut writer).unwrap();
    }
}
