use anyhow::{Context, Result};
use clap::Parser;
use indicatif::ProgressBar;
use log::{info, warn};
use std::{
    fs::read_to_string,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    path: PathBuf,
}
fn main() -> Result<()> {
    // env_logger::init();
    info!("starting up");
    warn!("Nothing done yet!");
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let args = Cli::parse();

    let pb = ProgressBar::new(100);
    let content = read_to_string(&args.path)
        .with_context(|| format!("could not load file at {}", &args.path.display()))?;
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(i);
    }
    writeln!(handle, "foo : {}", content)?;
    Ok(())
}
