use std::path::PathBuf;
use structopt::StructOpt;

mod statsgen;
mod unhex;
mod filtermask;
mod cgrams;

#[derive(StructOpt, Debug)]
#[structopt(name = "pack2")]
pub enum CmdOpts {
    /// Generates statistics from a [input] and writes masks to <output>
    /// stats are written to stderr
    #[structopt(name = "statsgen")]
    Statsgen {
        /// Input file, stdin if not present
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
        /// Output file, stdout if not present
        #[structopt(short, long, parse(from_os_str), display_order=1)]
        output: Option<PathBuf>,
        /// Separator used in mask output [default: TAB]
        #[structopt(short, long, display_order=2)]
        separator: Option<char>,
        /// Ignore passwords shorter than <min-length>
        #[structopt(long, default_value="1", display_order=13)]
        min_length: u16,
        /// Ignore passwords longer than <max-length>
        #[structopt(long, default_value="65535", display_order=14)]
        max_length: u16,
    },

    /// Decodes $HEX[] encoded lines
    #[structopt(name = "unhex")]
    Unhex {
        /// Input file, stdin if not present
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
        /// Output file, stdout if not present
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    },
    /// Filters a wordlist by a given mask
    #[structopt(name = "filtermask")]
    Filtermask {
        /// Mask to filter by
        #[structopt(required(true))]
        mask: String,
        /// Input file, stdin if not present
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
        /// Output file, stdout if not present
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
    },
    /// Splits each line on the charset boundry
    #[structopt(name = "cgrams")]
    Cgrams {
        /// Input file, stdin if not present
        #[structopt(parse(from_os_str))]
        input: Option<PathBuf>,
        /// Output file, stdout if not present
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        /// Sort by frequency (slower and memory hungry)
        #[structopt(short, long)]
        sort: bool,
    }
}

fn main() {
    let opt = CmdOpts::from_args();
    match opt {
        CmdOpts::Statsgen{ input, output, separator, min_length, max_length } => {
            statsgen::gen(input, output, separator, min_length, max_length);
        },
        CmdOpts::Unhex{ input, output } => {
            unhex::unhex(input, output);
        },
        CmdOpts::Filtermask{ input, output, mask } => {
            filtermask::filtermask(input, output, mask);
        },
        CmdOpts::Cgrams{ input, output, sort } => {
            cgrams::gen_c_grams(input, output, sort);
        }
    }
}
