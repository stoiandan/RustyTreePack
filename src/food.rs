use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy, Default, Debug)]
pub struct Food<'a> {
    pub name: &'a str,
    pub cost: u32,
    pub calories: u32
}

impl Display for Food<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}: {}€,  {}cal.", self.name, self.cost, self.calories)
    }
}