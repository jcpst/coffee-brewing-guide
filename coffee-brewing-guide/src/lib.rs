use measurements::{Mass, Volume};
use std::fmt;
use std::string::ToString;
use strum_macros::Display;

#[derive(Copy, Clone, Display, Eq, PartialEq)]
pub enum BrewTechnique {
    #[strum(serialize = "Pour Over")]
    PourOver,
    #[strum(serialize = "Aeropress")]
    AeroPress,
    #[strum(serialize = "Filtered Ice Coffee")]
    FilteredIceCoffee,
}

pub trait Brewable {
    /// Create a brew based on mL of water.
    fn from_water(self, amount: f64) -> Brew;
    /// Create a brew based on g of coffee beans.
    fn from_beans(self, amount: f64) -> Brew;
}

#[derive(Clone, Copy)]
pub struct Brew {
    pub water: Volume,
    pub beans: Mass,
}

impl fmt::Display for Brew {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.water, self.beans)
    }
}

/// Every `BrewTechnique` is `Brewable`
impl Brewable for BrewTechnique {
    fn from_water(self, water: f64) -> Brew {
        let beans = (water / 1000.) * self.grams_per_liter();
        Brew::new(water, beans)
    }

    fn from_beans(self, beans: f64) -> Brew {
        let water = 1000. / (self.grams_per_liter() / beans);
        Brew::new(water, beans)
    }
}

impl BrewTechnique {
    fn grams_per_liter(self) -> f64 {
        match self {
            BrewTechnique::PourOver => 60.,
            BrewTechnique::AeroPress => 55.,
            BrewTechnique::FilteredIceCoffee => 65.,
        }
    }

    pub fn get_guide(self, cups: u16, ml_per_cup: u16) -> Brew {
        let water = (cups * ml_per_cup) as f64;
        let beans = water * (self.grams_per_liter() / 1000.);
        Brew::new(water, beans)
    }
}

impl Brew {
    pub fn new(water: f64, beans: f64) -> Brew {
        Brew {
            water: Volume::from_milliliters(water),
            beans: Mass::from_grams(beans),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
