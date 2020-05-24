use std::io::{self};
use std::iter::FromIterator;
use std::path::PathBuf;

use bstr::{ByteSlice, io::BufReadExt};
use faster_hex::hex_encode;
use hashbrown::HashMap;
use pack2_util::*;

pub fn gen_c_grams(input: Option<PathBuf>, output: Option<PathBuf>, sort: bool) {

    let reader     = get_reader(input);
    let mut writer = get_writer(output);

    let mut c_gram   = [0u8; 0xffff];
    let mut out_hex = [0u8; 0xffff * 2];
    let mut c_grams: HashMap<Vec<u8>, u64> = HashMap::new();

    for result in reader.byte_lines() {
        let (line, line_len) = decode_hex_if_needed(result.unwrap());
        if line_len > u16::MAX.into() || line_len ==  0 { continue; }

        let mut last_charset: u8 = 0;
        let mut idx: usize = 0;

        for c in line.iter() {
            let cur_charset = CHAR2BITMAP[*c as usize];
            if last_charset == cur_charset {
                c_gram[idx] = *c;
                idx += 1;
                continue
            }
            if last_charset == 0 {
                c_gram[idx] = *c;
                idx += 1;
                last_charset = cur_charset;
                continue;
            }
            if last_charset != cur_charset {
                if sort {
                    *c_grams.entry(c_gram[..idx].to_vec()).or_insert(0) += 1;
                } else {
                    // $HEX[] encoding necessary
                    if last_charset == 16 {
                        let _ = hex_encode(&c_gram[..idx], &mut out_hex);
                        let out = &*format!("$HEX[{}]\n", &out_hex[..idx * 2].to_str().unwrap());
                        io::copy(&mut out.as_bytes(), &mut writer).unwrap();
                    } else {
                        c_gram[idx] = '\n' as u8;
                        io::copy(&mut c_gram[..=idx].as_bytes(), &mut writer).unwrap();
                    }
                }
                c_gram[0] = *c;
                idx = 1;
                last_charset = cur_charset;
            }

        }
        // TODO: duplicated code, find a better way to contruct the above loop
        if sort {
            *c_grams.entry(c_gram[..idx].to_vec()).or_insert(0) += 1;
        } else {
            // $HEX[] encoding necessary
            if last_charset == 16 {
                let _ = hex_encode(&c_gram[..idx], &mut out_hex);
                let out = &*format!("$HEX[{}]\n", &out_hex[..idx * 2].to_str().unwrap());
                io::copy(&mut out.as_bytes(), &mut writer).unwrap();
            } else {
                c_gram[idx] = '\n' as u8;
                io::copy(&mut c_gram[..=idx].as_bytes(), &mut writer).unwrap();
            }
        }
    }
    if sort {
        let mut freq_c_grams = Vec::from_iter(c_grams);
        freq_c_grams.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

        eprintln!("\n[*] c-grams (top 25):");
        let mut top = 0;

        for (c_gram, count) in freq_c_grams {
            let out = encode_hex_if_needed(c_gram);
            if top < 25 {
                eprintln!("[+] {: >26}: ({})", &out.to_str().unwrap(), count);
                top += 1;
            }
            let out = &*format!("{}\t{}\n", &out.to_str().unwrap(), count);
            io::copy(&mut out.as_bytes(), &mut writer).unwrap();
        }
    }
}
