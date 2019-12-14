use std::fmt::Display;

#[derive(Display)]
pub enum Role {
    Member,
    Admin,
    Moderator,
}

impl Role {
    pub fn parse(value: &str) -> Option<Role> {
        match value.to_lowercase().as_str() {
            "member" => Some(Role::Member),
            "admin" => Some(Role::Admin),
            "moderator" => Some(Role::Moderator),
            _ => None,
        }
    }
}
