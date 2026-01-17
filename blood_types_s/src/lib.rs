#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(self, other: Self) -> bool {
        let k = match self.antigen {
            Antigen::A => Antigen::A == other.antigen,
            Antigen::AB => true,
            Antigen::B => Antigen::B == other.antigen || Antigen::O == other.antigen,
            Antigen::O => Antigen::O == other.antigen,
        };
        let c = match self.rh_factor {
            RhFactor::Positive => true,
            RhFactor::Negative => RhFactor::Negative == other.rh_factor,
        };

        k && c
    }

    pub fn donors(self) -> Vec<Self> {
      oo().into_iter().filter(|c| self.can_receive_from(*c)).collect()
    }

    pub fn recipients(self) -> Vec<Self> {
              oo().into_iter().filter(|c| c.can_receive_from(self)).collect()

     
    }
}


fn  oo() ->Vec<BloodType>{

       vec![
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            }
        ]
}