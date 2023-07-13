xtern crate ini;
use ini::Ini;
use std::fs;
use std::io::{Write, stdout, stdin};
use std::process::{Command, Stdio};

fn main() {
    // Prompt user input 
    stdout().write_all(b"App: ").unwrap();
    stdout().flush().unwrap();
    let mut app = String::new();
    stdin().read_line(&mut app).expect("uhh");
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
            let binpath = exec.split_whitespace().next().unwrap();
            // Fix firefox/cura trying to open non-existant file
            if exec.contains('%') {
                let mut cmd = Command::new(binpath);
                cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
                break
            // If args...
            } else if exec.contains(' ') {
                let exec_split = exec.split_whitespace();
                let mut cmd = Command::new(binpath);
                cmd.args(exec_split).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
                break
            // If no args...
            }else {
                let mut cmd = Command::new(exec);
                cmd.stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
                break
            }
        }
    }   
}
