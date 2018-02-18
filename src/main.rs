extern crate docopt;
extern crate luxo;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
mod duration;
mod example;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = "
Luxo CLI

Usage:
  luxo stats <folder>
  luxo example <folder> [--store=<store>]
  luxo (-h | --help)
  luxo --version

Options:
  -h --help        Show this screen.
  --store=<store>  Type of store [default: simple].
  --version        Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    cmd_stats: bool,
    cmd_example: bool,
    flag_store: String,
    arg_folder: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some(VERSION.to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_stats {
        luxo::stats(&args.arg_folder)
    } else if args.cmd_example {
        example::example(&args.arg_folder, &args.flag_store)
    }
}