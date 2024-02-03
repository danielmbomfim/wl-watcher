use std::{
    path::{Path, PathBuf},
    process::Child,
    time::Duration,
};

use clap::Parser;
use notify_debouncer_full::{
    new_debouncer,
    notify::{RecursiveMode, Result, Watcher},
};

mod handlers;
use handlers::handle_file_modifications;

#[derive(Debug, Parser, Default)]
#[command(author = "Daniel M. Bomfim", version, about)]
#[command(
    help_template = "{name} \n{author} {about-section} \n {usage-heading} {usage} \n\n {all-args} {tab}"
)]
///Script simples para autodeploy de aplicações Weblogic
struct Args {
    #[arg(short, long)]
    ///Caminho para o diretório a ser observado
    source: PathBuf,

    #[arg(long, default_value = "1")]
    ///Tempo em segundos que o watcher aguarda por novas mudanças antes de iniciar o package
    delay: u64,

    #[arg(short = 't', long = "skip-tests", action, default_value = "false")]
    ///Define se os testes de vem ser executados no processo de package
    skip_tests: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(error) = watch(args.source) {
        println!("{:?}", error);
    }
}

fn watch<P: AsRef<Path>>(path: P) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(Args::parse().delay), None, tx)?;
    let mut previous_command: Option<Child> = None;

    debouncer
        .watcher()
        .watch(path.as_ref(), RecursiveMode::Recursive)?;
    debouncer
        .cache()
        .add_root(path.as_ref(), RecursiveMode::Recursive);

    for res in rx {
        match res {
            Ok(events) => {
                previous_command = handle_file_modifications(events, previous_command);
            }
            Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        }
    }

    Ok(())
}
