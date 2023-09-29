use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("StableSwap", "abi/stableswap.json")?
        .generate()?
        .write_to_file("src/abi/stable_swap.rs")?;

    Ok(())
}
