const MAX_LEN: usize = 255;
const MIN_LEN: usize = 1;

pub fn username_min_len(u: &str) -> Result<(), String> {
    if u.len() >= MIN_LEN {
        Ok(())
    } else {
        Err(format!(
            "Username too short. Min length is {MIN_LEN}, username is {}\n",
            u.len()
        ))
    }
}

pub fn username_max_len(u: &str) -> Result<(), String> {
    if u.len() <= MAX_LEN {
        Ok(())
    } else {
        Err(format!(
            "Username too long. Max length is {MAX_LEN}, username is {}\n",
            u.len()
        ))
    }
}

pub fn no_starting_space(u: &str) -> Result<(), String> {
    if u.starts_with(' ') {
        Err(format!("Username should not start with a space\n"))
    } else {
        Ok(())
    }
}

pub fn password_len(p: &str) -> Result<(), String> {
    if p.is_empty() {
        return Err("Password can not be empty\n".to_string());
    }

    if p.len() > MAX_LEN {
        return Err(format!(
            "Password can not be longer than {MAX_LEN}, password is {}\n",
            p.len()
        ));
    }

    Ok(())
}
