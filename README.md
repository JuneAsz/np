

`np` is a simple command-line tool to quickly initialize new projects in **Rust** or **Golang**. It creates the project directory, initializes the project with the proper tool (`cargo` for Rust, `go mod init` for Go), and generates basic build and run scripts.

**Note:** This tool is not meant to be used seriously, this is just for personal convenience.

## Installation

1. Clone this repository:

git clone https://github.com/JuneAsz/np 

cd np

2. Build the tool with Cargo:

cargo build --release

3. Add the compiled binary to your PATH or run it from the target directory:

./target/release/np

---

## Usage

np <project_name> <language_type>

### Arguments

- `project_name` – Name of the new project.
- `language_type` – Programming language for the project. Options:
    - `rust` (alias `rs`)
    - `golang` (alias `go`)

### Example

np my_rust_app rust  
np my_go_app go

This will:

1. Create a directory with the project name.
2. CD into said directory.
3. Initialize a new Rust or Go project.
4. Generate `build.bat` and `run.bat` for easy building and running.

---

## Generated Files

- `build.bat` – Builds the project (`cargo build --release` for Rust, `go build .` for Go)
- `run.bat` – Runs the project (`cargo run` for Rust, `go run .` for Go)

---

## Requirements

- Rust (with Cargo) installed for Rust projects.
- Go installed for Go projects.
- Windows (for `.bat` scripts). Scripts can be modified for other platforms.

---
