use std::{path, process::Command};
use notify_rust::{Notification, Hint};


fn run_script(){
    let path = "/usr/local/bin/system-maintenance";
    let command = format!("pkexec bash -c '{}; exec bash'", path);
    let output =  Command::new("konsole")
        .arg("--hold")
        .arg("-e")
        .arg("bash")
        .arg("-c")
        .arg(&command)
        .spawn();
    
}
fn main() {
    Notification::new()
        .summary("Arch Update Reminder")
        .action("update", "Update Now")
        .hint(Hint::Resident(true))
        .timeout(0)  
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "update" => {
                run_script(); 
                std::process::exit(0);
            },
            _ => std::process::exit(0)
        });

    
}

