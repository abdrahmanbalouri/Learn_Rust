use std::fmt;

pub struct Park {
    pub name: Option<String>,
    pub park_type: ParkType,
    pub address: Option<String>,
    pub cap: Option<String>,
    pub state: Option<String>,
}

pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name.as_deref().unwrap_or("no name");
        let address = self.address.as_deref().unwrap_or("no address");

        let cap = self.cap.as_deref().unwrap_or("no cap");

        let state = self.state.as_deref().unwrap_or("no state");
        write!(f,"{} - {}, {}, {} - {}",self.park_type,name,address,cap,state)
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let k = match self{
            ParkType::Garden =>"garden",
            ParkType::Forest=> "forest",
            ParkType::Playground=>"playground",
        };
        write!(f,"{}",k)
    }
}
