use std::collections::HashMap;

pub fn helper_string() -> HashMap<String, String> {
    return HashMap::from([
        ("s".to_string(), "[S] Start launcher".to_string()),
        ("e".to_string(), "[E] Edit instance".to_string()),
        ("0".to_string(), "[0] Exit".to_string())
    ]);
}
