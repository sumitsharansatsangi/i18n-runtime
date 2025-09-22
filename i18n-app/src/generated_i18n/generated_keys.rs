// GENERATED — DO NOT EDIT

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageKey {
    Welcome, // id = 0
    LoginSuccess, // id = 1
    LoginFailed, // id = 2
    Error, // id = 3
}

impl MessageKey {
    pub fn as_u32(self) -> u32 {
        match self {
            MessageKey::Welcome => 0,
            MessageKey::LoginSuccess => 1,
            MessageKey::LoginFailed => 2,
            MessageKey::Error => 3,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            MessageKey::Welcome => "welcome",
            MessageKey::LoginSuccess => "login_success",
            MessageKey::LoginFailed => "login_failed",
            MessageKey::Error => "error",
        }
    }

    pub fn from_u32(v: u32) -> Option<MessageKey> {
        match v {
            0 => Some(MessageKey::Welcome),
            1 => Some(MessageKey::LoginSuccess),
            2 => Some(MessageKey::LoginFailed),
            3 => Some(MessageKey::Error),
            _ => None,
        }
    }
}
