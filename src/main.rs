use std::path::PathBuf;
use structopt::StructOpt;

mod statsgen;

#[derive(StructOpt, Debug)]
#[structopt(name = "pack2")]
pub enum CmdOpts {
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
        /// Ignore passwords shorter than min-length
        #[structopt(long, default_value="1", display_order=13)]
        min_length: u16,
        /// Ignore passwords longer than max-length
        #[structopt(long, default_value="65535", display_order=14)]
        max_length: u16,
    },
}

fn main() {
    let opt = CmdOpts::from_args();
    match opt {
        CmdOpts::Statsgen{ input, output, separator, min_length, max_length } => {
            statsgen::gen(input, output, separator, min_length, max_length);
        }
    }

}
