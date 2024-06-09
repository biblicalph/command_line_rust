use anyhow;
use args::Wcr;
use clap::Parser;

mod args;
mod counts;

pub fn run() -> anyhow::Result<()> {
    let wcr = Wcr::parse().set_defaults();

    let res = counts::compute(&wcr)?;
    res.print();
    Ok(())
}
