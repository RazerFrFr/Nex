use colored::Colorize;
use std::net::TcpStream;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use std::{env, thread};

pub fn init_backend(backend: &str) -> Option<Child> {
    let current_dir = env::current_dir().unwrap();
    let mut backend_path = current_dir.clone();
    backend_path.push("backends");
    backend_path.push(backend);
    backend_path.push("run.bat");

    if !backend_path.exists() {
        println!("{} {}", "run.bat not found for backend:".red(), backend);
        return None;
    }

    let backend_dir = current_dir.join("backends").join(backend);

    let mut cmd = Command::new("cmd");
    cmd.args(["/C", "start", "\"\"", "/B", backend_path.to_str().unwrap()])
        .current_dir(backend_dir)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    match cmd.spawn() {
        Ok(child) => Some(child),
        Err(e) => {
            println!("{} {}", "Failed to start backend:".red(), e);
            None
        }
    }
}

pub fn wait_for_backend(addr: &str) {
    loop {
        if TcpStream::connect(addr).is_ok() {
            break;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
