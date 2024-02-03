use std::process::{Child, Command};

use clap::Parser;
use notify_debouncer_full::DebouncedEvent;

use crate::Args;

pub fn handle_file_modifications(
    events: Vec<DebouncedEvent>,
    previous_command: Option<Child>,
) -> Option<Child> {
    let mut code_changed = false;

    for evt in events.iter() {
        if code_changed {
            break;
        }

        evt.paths.iter().for_each(|path| {
            let filepath = path.to_str().unwrap_or("");

            if !code_changed && (filepath.contains(".java") || filepath.contains("pom.xml")) {
                code_changed = true;
                return;
            }
        });
    }

    if code_changed {
        match previous_command {
            Some(mut process) => {
                println!("Clearing previous processess");

                if let Err(error) = process.kill() {
                    println!("{}", error.to_string());
                }

                if let Err(error) = process.wait() {
                    println!("{}", error.to_string());
                }
            }
            None => {
                println!("No previous process");
            }
        };

        let process = package_project(&format_pom_path(Args::parse().source.to_str().unwrap()));
        return Some(process);
    }

    previous_command
}

fn package_project(pom_path: &str) -> Child {
    let handler = Command::new("mvn")
        .arg("package")
        .arg("-f")
        .arg(pom_path)
        .arg(format!(
            "-Dmaven.test.skip.exec={}",
            !Args::parse().skip_tests
        ))
        .spawn()
        .expect("failed to execute package command");

    handler
}

fn format_pom_path(path: &str) -> String {
    if !path.ends_with("/") {
        return format!("{}/pom.xml", path);
    }

    return format!("{}pom.xml", path);
}
