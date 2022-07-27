use anyhow::{bail, Result};
use clap::{Args, Parser, Subcommand};
use rust_chess::chess::{Game, STARTING_POS};

#[derive(Debug, Parser)]
struct ChessArgs {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
    Perft(PerftCommand),
    Play(PlayCommand),
}

#[derive(Debug, Args)]
struct PerftCommand {
    fen: String,
    depth: u32,
}

#[derive(Debug, Args)]
struct PlayCommand {
    fen: String,
}

fn main() -> Result<()> {
    let args = ChessArgs::parse();
    match &args.action {
        Action::Perft(cmd) => {
            let nodes = if cmd.fen.eq("startpos") {
                Game::perft(STARTING_POS, cmd.depth)
            } else {
                Game::perft(&cmd.fen, cmd.depth)
            };
            match nodes {
                Ok(n) => {
                    println!("{n}");
                    Ok(())
                }
                Err(err) => bail!(err),
            }
        }
        Action::Play(_) => Ok(()),
    }
}
