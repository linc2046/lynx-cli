use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Lynx-Cli", about = "Cli for the Lynx Language")]
struct Opt {    
    /// Run single lynx file
    // short and long flags (-r, --run)
    #[structopt(short, long)]
    run: bool,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}