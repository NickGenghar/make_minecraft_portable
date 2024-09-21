use std::process::Command;

pub struct MMcP {
    pub instance_name: String,
    pub instance_dir: String,
    pub minecraft: Command,
}

impl MMcP {
    pub fn set_instance_name(&mut self, name:String) {
        self.instance_name = name;
    }

    pub fn check_variable(&mut self) {
        if self.instance_dir != "" {
            self.minecraft
                .arg(format!("--workdir={}", &self.instance_dir));
            println!("Instance dir set: {}", self.instance_dir);
        } else {
            println!("No instance dir set. Using default.");
        }
    }

    pub fn exec(&mut self) {
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