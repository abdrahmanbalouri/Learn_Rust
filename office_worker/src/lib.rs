#[derive(Debug, PartialEq, Eq)]
pub struct OfficeWorker {
   pub name: String,
   pub age: u32,
   pub    role: WorkerRole,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkerRole {
    Admin,
    User,
    Guest,
}

impl From<&str> for OfficeWorker {
    fn from(s: &str) -> Self {
        let b: Vec<&str> = s.split(",").collect();
        let k = match b[2] {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => WorkerRole::Guest,
        };

        Self {
            name: (b[0].to_string()),
            age: (b[1].parse().unwrap()),
            role: (k),
        }
    }
}

impl From<&str> for WorkerRole {
    fn from(s: &str) -> Self {

        let k = match s {
            "admin" => WorkerRole::Admin,
            "user" => WorkerRole::User,
            "guest" => WorkerRole::Guest,
            _ => WorkerRole::Guest,
        };
        k
    }
}
