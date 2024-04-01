mod analyze;
mod common;
mod simulate;

use self::analyze::*;
use self::common::*;
use self::simulate::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cmd {
    Analyze(AnalyzeCmd),
    Simulate(SimulateCmd),
}

fn main() {
    match Cmd::from_args() {
        Cmd::Analyze(cmd) => cmd.run(),
        Cmd::Simulate(cmd) => cmd.run(),
    }
}
