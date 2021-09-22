//! Structures holding recipe data
use std::fmt;

use crate::measure::{Mass, Volume};

/// One ingredient in a recipe, with amount of the ingredient and ingredient name
#[derive(Clone, Debug, )]
pub struct Ingredient {
    pub name: String,
    pub amount: IngredientAmount,
}

/// Enumeration for how an ingredient's amount is displayed
#[derive(Clone, Debug, )]
pub enum IngredientAmount {
    /// A raw number, displayed as x{n}
    Count(usize),
    /// A measurement of volume in cups, liters, etc.
    Volume(Volume),
    /// A measurement in mass
    Mass(Mass),
    /// No amount given
    None,
}   

impl fmt::Display for IngredientAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Count(amt) => write!(f, "x{}", amt),
            Self::Volume(vol) => vol.fmt(f),
            Self::Mass(mass) => mass.fmt(f),
            Self::None => Ok(())
        }
    }
}

/// Struct containing all data a user can add to a recipe
#[derive(Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Option<Vec<Ingredient>>,
    pub body: String,
}
