use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

include!("src/cli_args.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = Cli::command();
    for shell in [Shell::Bash, Shell::Zsh] {
        let path = generate_to(
            shell,
            &mut cmd,      // We need to specify what generator to use
            "emojfuscate", // We need to specify the bin name manually
            &outdir,       // We need to specify where to write to
        )?;

        println!("cargo:warning={shell:?} completion file is generated: {path:?}");
    }

    Ok(())
}
