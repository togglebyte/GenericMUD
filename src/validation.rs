use std::fmt::Display;

const MAX_LEN: usize = 255;
const MIN_LEN: usize = 1;

#[derive(Debug)]
pub enum ValidationErr {
    UsernameTooLong { current: usize, max: usize },
    UsernameTooShort { current: usize, min: usize },
    UsernameStartsWithSpace,
    PasswordTooLong { current: usize, max: usize },
    PasswordTooShort { current: usize, min: usize },
    PasswordEmpty,
    ContainsNonAscii,
    ContainsSpaces,
}

impl Display for ValidationErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UsernameTooLong { current, max } => write!(
                f,
                "Username is too long, max {max}, current username is {current}."
            ),
            Self::UsernameTooShort { current, min } => write!(
                f,
                "Username is too short, min {min}, current username is {current}."
            ),
            Self::UsernameStartsWithSpace => write!(f, "Username can't start with a space"),
            Self::PasswordTooLong { current, max } => write!(
                f,
                "Password is too long, max {max}, current password is {current}."
            ),
            Self::PasswordTooShort { current, min } => write!(
                f,
                "Password is too short, min {min}, current password is {current}."
            ),
            Self::PasswordEmpty => write!(f, "Password can not be empty"),
            Self::ContainsNonAscii => write!(f, "Input contains non-ascii characters"),
            Self::ContainsSpaces => write!(f, "Input contains whitespace"),
        }
    }
}

pub fn no_starting_space(u: &str) -> Result<(), ValidationErr> {
    match u.starts_with(' ') {
        true => Err(ValidationErr::UsernameStartsWithSpace),
        false => Ok(()),
    }
}

pub fn username_len(u: &str) -> Result<(), ValidationErr> {
    if u.len() < MIN_LEN {
        return Err(ValidationErr::UsernameTooShort {
            current: u.len(),
            min: MIN_LEN,
        });
    }

    if u.len() > MAX_LEN {
        return Err(ValidationErr::UsernameTooLong {
            current: u.len(),
            max: MAX_LEN,
        });
    }

    Ok(())
}

pub fn password_len(p: &str) -> Result<(), ValidationErr> {
    if p.is_empty() {
        return Err(ValidationErr::PasswordEmpty);
    }

    if p.len() > MAX_LEN {
        return Err(ValidationErr::PasswordTooLong {
            current: p.len(),
            max: MAX_LEN,
        });
    }

    if p.len() < MIN_LEN {
        return Err(ValidationErr::PasswordTooShort {
            current: p.len(),
            min: MIN_LEN,
        });
    }

    Ok(())
}

pub fn ascii_only(input: &str) -> Result<(), ValidationErr> {
    if !input.is_ascii() {
        return Err(ValidationErr::ContainsNonAscii);
    }

    Ok(())
}

pub fn no_space(input: &str) -> Result<(), ValidationErr> {
    if input.contains(char::is_whitespace) {
        return Err(ValidationErr::ContainsNonAscii);
    }

    Ok(())
}
