//demoapp
use std::io::{self};
use std::process::Command;
use std::env::{self};
use std::os::{self};

const HELPER_STRING:[&str; 6] = [
    "[S] Start launcher",
    "[E] Edit instance",
    "",
    "[0] Exit",
    "",
    "Enter selection: "
];

struct MMcP {
    instance_name: String,
    instance_dir: String,
    minecraft: Command,
}

impl MMcP {
    fn set_instance_name(&mut self, name:String) {
        self.instance_name = name;
    }

    fn check_variable(&mut self) {
        if self.instance_dir != "" {
            self.minecraft
                .arg(format!("--workdir={}", &self.instance_dir));
            println!("Instance dir set: {}", self.instance_dir);
        } else {
            println!("No instance dir set. Using default.");
        }
    }

    fn exec(&mut self) {
        match self.minecraft.output() {
            Ok(_) => {
                println!("Launcher closed. Returning to MMcP.");
            },
            Err(e) => {
                println!("An error occured while starting Minecraft. See error message for help:\n\n {}", e);
            }
        }
    }
}

fn modify_mmcp(mmcp: &mut MMcP, buf: &mut String) {
    loop {
        print!("Name:\t");
        match io::stdin().read_line(buf) {
            Ok(_) => {
                mmcp.set_instance_name(buf.to_string());
                buf.clear();
            },
            Err(e) => {
                println!("An error occurred:\n{}", e);
                buf.clear();
                continue;
            }
        };
        print!("Location:\t");
        io::stdin().read_line(buf).expect("Failure to parse location.");
    }
}

fn main() {
    //Linux only for now, so let's exit the program immediately if it's
    //built and ran on other operating system...
    if !cfg!(target_os = "linux") {
        return
    };
    //setup
    let minecraft = Command::new("minecraft-launcher");
    let mut buf = String::new();

    let mut mmcp = MMcP {
        instance_name: String::from("Vanilla"),
        instance_dir: String::from(""),
        minecraft: minecraft
    };

    println!("MMcP - Make Minecraft Portable");

    loop {
        for i in HELPER_STRING {
            println!("{}", i);
        }

        buf.clear();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                if buf.contains('S') || buf.contains('s') {
                    println!("Starting launcher, please wait.");
                    mmcp.exec();
                } else if buf.contains('E') || buf.contains('e') {
                    println!("Function not implemented to the fullest. Instead, it will show the current installation status.");
                    mmcp.check_variable();
                } else if buf.contains('0') {
                    println!("Thank you for using MMcP.");
                    break;
                } else {
                    println!("Invalid input, try again.");
                }
            },
            Err(e) => {
                println!("No input provided (somehow?).\n{}", e);
            }
        }
    }
}