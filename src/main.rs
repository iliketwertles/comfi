extern crate ini;
use ini::Ini;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() {
    // Prompt user input 
    io::stdout().write_all(b"App: ").unwrap();
    io::stdout().flush().unwrap();
    let mut app = String::new();
    io::stdin().read_line(&mut app).expect("uhh");
    let len = app.len();
    app.truncate(len - 1);
    // Path to .desktop files
    let paths = fs::read_dir("/usr/share/applications/").unwrap();
    for path in paths {
        // Parse user input
        if path.as_ref().unwrap().path().display().to_string().contains(&app) {
            let full_path = &path.unwrap().path().display().to_string();
            let app_info = Ini::load_from_file(full_path).unwrap();
            // Parse .desktop file
            let data = app_info.section(Some("Desktop Entry")).unwrap();
            let exec = data.get("Exec").unwrap();
            // Run exec with args...
            if exec.contains(' ') {
                let mut exec_split = exec.split_whitespace();
                let mut cmd = Command::new(exec_split.next().unwrap());
                cmd.args(exec_split).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
            // If no args...
            } else {
                let mut cmd = Command::new(exec);
                cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
            }
        }
    }   
}