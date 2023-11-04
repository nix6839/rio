use clap::CommandFactory;
use clap_complete::{generate_to, shells::Shell as ClapShell};

include!("./src/cli.rs");

fn main() {
    let mut cmd = Options::command();
    let shells = [ClapShell::Bash, ClapShell::Zsh, ClapShell::Fish];

    for shell in shells {
        generate_to(shell, &mut cmd, "rio", "./completions").unwrap();
    }
}
