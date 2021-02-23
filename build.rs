use std::fs;
use clap::Shell;

include!("src/bruteforus/app.rs");

fn main() {
    let var = std::env::var_os("SHELL_COMPLETIONS_DIR").or(std::env::var_os("OUT_DIR"));
    let outdir = match var {
        None => return,
        Some(outdir) => outdir,
    };
    fs::create_dir_all(&outdir).unwrap();

    let mut app = build();
    app.gen_completions("bruteforus", Shell::Bash, &outdir);
    app.gen_completions("bruteforus", Shell::Fish, &outdir);
    app.gen_completions("bruteforus", Shell::Zsh, &outdir);
    app.gen_completions("bruteforus", Shell::PowerShell, &outdir);
}