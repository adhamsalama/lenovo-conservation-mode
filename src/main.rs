use std::process::Command;
use std::{env, fs, process::exit};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Wrong number of args");
        exit(1);
    }
    let command = &args[0].to_lowercase()[..];
    let path = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";
    let on_command = "echo 1 > ".to_owned() + path;
    let off_command = "echo 0 > ".to_owned() + path;
    match command {
        "on" => {
            Command::new("sudo")
                .arg("su")
                .arg("-c")
                .arg(on_command)
                .output()
                .unwrap();
        }
        "off" => {
            Command::new("sudo")
                .arg("su")
                .arg("-c")
                .arg(off_command)
                .output()
                .unwrap();
        }
        "show" => {
            let status = fs::read_to_string(path).unwrap();
            if status.trim() == "1" {
                println!("Conservation mode is ON");
            } else {
                println!("Conservation mode is OFF");
            }
        }
        _ => {
            println!("Unsupported command.\nSupported commands are on/off/show");
        }
    }
}
