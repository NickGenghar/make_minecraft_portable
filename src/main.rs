use std::io::{self};
use std::process::Command;
use std::fs::{self};

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
                if cfg!(target_os = "linux") {
                    println!("Launcher closed. Returning to MMcP.");
                } else if cfg!(target_os = "windows") {
                    //The Windows version of the launcher somehow disowned the spawned
                    //child process MMcP requires. This message here will act as a notice
                    //until a better implementation made.
                    println!("MMcP window will stay open while the launcher is opened. Be sure to close the launcher first before doing any other tasks.");
                }
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
    //Welcome message.
    println!("MMcP - Make Minecraft Portable");

    //setup
    let minecraft:Command;
    let launcher_dir:String;

    if cfg!(target_os = "windows") {
        launcher_dir = String::from("C:\\XboxGames\\Minecraft Launcher\\Content\\Minecraft.exe");
    } else if cfg!(target_os = "linux") {
        launcher_dir = String::from("minecraft-launcher");
    } else {
        println!("Hello there, MMcP does not support your platform. Stay tuned for more info.");
        return;
    };

    //Making sure Minecraft Launcher is installed before proceeding.
    //There will be, issues, if the launcher isn't installed yet...
    match fs::exists(&launcher_dir) {
        Ok(r) => {
            if r {
                minecraft = Command::new(&launcher_dir);
            } else {
                println!("There was an issue picking up the launcher. MMcP does not implemented a function to auto install, therefore it could not proceed any further. Sorry for the inconvenience.");
                return;
            }
        },
        Err(_) => {
            println!("Launcher installation not found. MMcP does not implement the necessary features to download and install the launcher yet. Stay tuned for more info.");
            return;
        }
    }
    let mut buf = String::new();

    let mut mmcp = MMcP {
        instance_name: String::from("Vanilla"),
        instance_dir: String::from(""),
        minecraft: minecraft
    };

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