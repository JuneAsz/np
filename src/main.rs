use std::{fs::create_dir, path::Path, path::PathBuf, process::Command};

use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    project_name: String,

    #[arg(value_enum)]
    language_type: LanguageType,
}

#[derive(Debug, Clone, ValueEnum)]
enum LanguageType {
    #[clap(alias = "rs")]
    Rust,
    #[clap(alias = "go")]
    Golang,
}

impl LanguageType {
    fn init(&self, project_name: String) {
        let status = match self {
            LanguageType::Rust => Command::new("cargo")
                .arg("init")
                .status()
                .expect("failed to run cargo"),
            LanguageType::Golang => Command::new("go")
                .arg("mod")
                .arg("init")
                .arg(project_name)
                .status()
                .expect("failed to run go"),
        };

        if !status.success() {
            eprintln!("Command failed with exit code {:?}", status.code());
            std::process::exit(status.code().unwrap_or(1));
        }
    }
}

fn parse_args() -> (String, LanguageType) {
    let args = Args::parse();

    return (args.project_name, args.language_type);
}

fn process(project_name: String, language_type: LanguageType) -> Result<(), std::io::Error> {
    let path = Path::new(&project_name);
    std::fs::create_dir(&path)?;
    std::env::set_current_dir(&path)?;

    language_type.init(project_name);

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let (project_name, language_type) = parse_args();
    process(project_name, language_type)?;

    Ok(())
}
