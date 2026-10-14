use crate::mmcp::MMcP;

pub struct Tui {
    pub option_id: String,
    pub option_name: String,
    pub option_reply: String,
    pub action: Box<dyn FnMut(&mut MMcP)>,
}

impl Tui {
    pub fn new(option_id: String, option_name: String, option_reply: String, action: impl FnMut(&mut MMcP) + 'static) -> Self {
        Tui {
            option_id,
            option_name,
            option_reply,
            action: Box::new(action),
        }
    }

    pub fn helper_string() -> Vec<Self> {
        return vec![
            Self::new(
                "s".to_string(),
                "[S] Start launcher".to_string(),
                "Starting launcher, please wait.".to_string(),
                |mmcp| mmcp.exec()
            ),
            Self::new(
                "e".to_string(),
                "[E] Edit instance".to_string(),
                "Function not implemented to the fullest. Instead, it will show the current installation status.".to_string(),
                |mmcp| mmcp.check_variable()
            ),
            Self::new(
                "0".to_string(),
                "[0] Exit".to_string(),
                "Thank you for using MMcP.".to_string(),
                |mmcp| mmcp.lifetime = 0
            )
        ];
    }

    pub fn handle_elements(mmcp: &mut MMcP, buf: &mut String) {
        for tui in Self::helper_string() {
            if buf.to_lowercase().contains(&tui.option_id.to_lowercase()) {
                println!("{}", tui.option_reply);
                (Box::new(tui.action))(mmcp);
                return;
            }
        }
        println!("Invalid input, try again.");
    }
}