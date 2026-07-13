use std::error::Error;
use std::io;

pub fn get_user_input() -> Result<String, Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

pub fn get_yes_no() -> Result<bool, Box<dyn Error>> {
    let user_input = get_user_input()?;
    let user_input = user_input.trim();
    if user_input == "Y" || user_input == "y" {
        Ok(true)
    } else {
        Ok(false)
    }
}
