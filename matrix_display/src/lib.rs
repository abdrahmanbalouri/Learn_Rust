use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let k = slice.iter().map(|x| x.to_vec()).collect();
        return Self(k);
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let max = self.0.len();
        let mut ind = 0;
        for i in &self.0{
            write!(f, "(")?;

            for (v, c) in i.iter().enumerate() {
                if v > 0 {
                    write!(f, " ")?;
                }

                write!(f, "{}", c)?;
            }
            write!(f, ")")?;
            ind+=1;
            if ind !=max{

                writeln!(f)?;
            }
        }
        Ok(())
    }
}
