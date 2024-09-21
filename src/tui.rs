use std::collections::HashMap;

use crate::mmcp::MMcP;

pub fn helper_string(mmcp:&mut MMcP) -> HashMap<String, (String, String, ())> {
    return HashMap::from([
        ("s".to_string(), (
            "[S] Start launcher".to_string(),
            "Starting launcher, please wait.".to_string(),
            mmcp.exec()
        )),
        ("e".to_string(), (
            "[E] Edit instance".to_string(),
            "Function not implemented to the fullest. Instead, it will show the current installation status.".to_string(),
            mmcp.check_variable()
        )),
        ("0".to_string(), (
            "[0] Exit".to_string(),
            "Thank you for using MMcP.".to_string(),
            ()
        ))
    ]);
}

//WARNING!
//Code is buggy, do not use.
//Fix is planned for future commits.
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
}