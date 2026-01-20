pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    pub name: String,
    pub level: AccessLevel,
}

impl User {
    pub fn new(name: String, level: AccessLevel) -> Self {
        Self {
            name: name,
            level: level,
        }
    }

    pub fn send_name(&self) -> Option<&str> {
        match self.level {
            AccessLevel::Guest => None,
            AccessLevel::Normal => Some(&self.name),
            AccessLevel::Admin => Some(&self.name),
        }
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
     let k  = user.send_name();
     if k.is_none(){
        return (false , "ERROR: User is guest");
    }
    let c = k.unwrap();
    (true , c)
}
