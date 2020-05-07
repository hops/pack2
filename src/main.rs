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
        #[structopt(short, long, parse(from_os_str))]
        output: Option<PathBuf>,
        /// Separator used in mask output [default: TAB]
        #[structopt(short, long)]
        separator: Option<char>
    },
}

fn main() {
    let opt = CmdOpts::from_args();
    match opt {
        CmdOpts::Statsgen{ input, output, separator } => {
            statsgen::gen(input, output, separator);
        }
    }

}
