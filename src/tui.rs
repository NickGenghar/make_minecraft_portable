use std::collections::HashMap;

use crate::mmcp::MMcP;

pub struct Tui {
    pub option_name: String,
    pub option_reply: String,
    pub action: Box<dyn FnMut(&mut MMcP)>,
}

impl Tui {
    pub fn new(name: String, reply: String, action: impl FnMut(&mut MMcP) + 'static) -> Self {
        Tui {
            option_name: name,
            option_reply: reply,
            action: Box::new(action),
        }
    }

    pub fn helper_string() -> HashMap<String, Self> {
        return HashMap::from([
            ("s".to_string(), Self::new(
                "[S] Start launcher".to_string(),
                "Starting launcher, please wait.".to_string(),
                |mmcp| mmcp.exec()
            )),
            ("e".to_string(), Tui::new(
                "[E] Edit instance".to_string(),
                "Function not implemented to the fullest. Instead, it will show the current installation status.".to_string(),
                |mmcp| mmcp.check_variable()
            )),
            ("0".to_string(), Tui::new(
                "[0] Exit".to_string(),
                "Thank you for using MMcP.".to_string(),
                // empty closure, as we don't need to do anything when exiting.
                |_mmcp| {}
            ))
        ]);
    }
}

/*WARNING!
//Code is buggy, do not use.
//Fix is planned for future commits as well as integrating into the
//Tui struct.
pub fn handle_elements(mmcp:&mut MMcP, buf:&mut String) {
    for s in helper_string(mmcp).keys() {
        if buf.to_lowercase().contains(s) {
            match helper_string(mmcp).get(s) {
                Some(e) => {
                    println!("{}", e.1);
                    e.2;
                }
                None => {}
            }
        } else {
            println!("Invalid input, try again.");
        }
    }
}*/