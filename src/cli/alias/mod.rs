use clap::Subcommand;
use color_eyre::eyre::Result;

use crate::cli::command::Command;
use crate::config::Config;
use crate::output::Output;
use crate::plugins::PluginName;

mod ls;

#[derive(Debug, clap::Args)]
#[clap(about = "Manage aliases", visible_alias = "a", alias = "aliases")]
pub struct Alias {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// filter aliases by plugin
    #[clap(short, long)]
    pub plugin: Option<PluginName>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ls(ls::AliasLs),
}

impl Commands {
    pub fn run(self, config: Config, out: &mut Output) -> Result<()> {
        match self {
            Self::Ls(cmd) => cmd.run(config, out),
        }
    }
}

impl Command for Alias {
    fn run(self, config: Config, out: &mut Output) -> Result<()> {
        let cmd = self.command.unwrap_or(Commands::Ls(ls::AliasLs {
            plugin: self.plugin,
        }));

        cmd.run(config, out)
    }
}
