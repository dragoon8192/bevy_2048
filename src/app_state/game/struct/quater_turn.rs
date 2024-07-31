use std::fmt::Debug;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuarterTurn {
    #[default]
    Deg000,
    Deg090,
    Deg180,
    Deg270,
}

impl QuarterTurn {
    pub fn downward_unit(&self) -> (isize, isize) {
        match self {
            QuarterTurn::Deg000 => return (0, -1),
            QuarterTurn::Deg090 => return (-1, 0),
            QuarterTurn::Deg180 => return (0, 1),
            QuarterTurn::Deg270 => return (1, 0),
        }
    }
}
