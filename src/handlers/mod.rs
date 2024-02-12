use std::{
    fs,
    path::PathBuf,
    process::{Child, Command},
};

use clap::Parser;
use notify_debouncer_full::DebouncedEvent;

use crate::Args;

pub fn source_modifications_handler(
    events: Vec<DebouncedEvent>,
    previous_command: Option<Child>,
) -> Option<Child> {
    let mut code_changed = false;
    let mut war_file: Option<String> = None;

    for evt in events.iter() {
        evt.paths.iter().for_each(|path| {
            let filepath = path.to_str().unwrap_or("");

            if !code_changed && (filepath.ends_with(".java") || filepath.ends_with("pom.xml")) {
                code_changed = true;
            }

            if filepath.ends_with(".war") && evt.kind.is_modify() {
                if let Some(value) = path.to_str() {
                    war_file = Some(value.to_owned());
                }
            }
        });
    }

    if let Some(path) = war_file {
        deploy_application(&path);
    }

    if code_changed {
        clear_previous_processes(previous_command);

        let process = package_project(&format_file_path(
            Args::parse().source.to_str().unwrap(),
            "pom.xml",
        ));
        return Some(process);
    }

    previous_command
}

fn package_project(pom_path: &str) -> Child {
    Command::new("mvn")
        .arg("package")
        .arg("-f")
        .arg(pom_path)
        .arg(format!(
            "-Dmaven.test.skip.exec={}",
            !Args::parse().skip_tests
        ))
        .spawn()
        .expect("failed to execute package command")
}

fn deploy_application(path: &str) {
    let args = Args::parse();

    println!("Copying {} to {:?}", path, args.deploypath);

    let mut process = Command::new("cp")
        .arg(path)
        .arg(format_file_path(
            Args::parse().deploypath.to_str().unwrap(),
            "",
        ))
        .spawn()
        .expect("failed to deploy war file");

    let _ = process.wait();
}

fn format_file_path(path: &str, file: &str) -> String {
    if !path.ends_with("/") {
        return format!("{}/{}", path, file);
    }

    return format!("{}{}", path, file);
}

fn clear_previous_processes(previous_process: Option<Child>) {
    println!("==>> Clearing previous processes");

    match previous_process {
        Some(mut process) => {
            if let Err(error) = process.kill() {
                println!("{}", error.to_string());
            }

            if let Err(error) = process.wait() {
                println!("{}", error.to_string());
            }
        }
        None => (),
    }
}

pub fn clear_deploy_dir(path: PathBuf) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();

        if file_name.ends_with(".war") {
            println!("==>> Removing {}...", file_name);
            fs::remove_file(entry.path()).unwrap();
        }
    }
}
