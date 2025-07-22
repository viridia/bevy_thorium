use bevy::prelude::*;

mod calc;
mod computations;
mod effect_cell;
mod insert_when;
mod signal;
// mod switch;

pub use calc::{Calc, calc};
pub use computations::{ComputationOf, Computations};
pub use insert_when::{InsertWhen, InsertWhenId, insert_when, insert_when_id};
// pub use switch::switch;

use crate::effect_cell::update_effects;

pub struct ThoriumPlugin;

impl Plugin for ThoriumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
        // app.add_systems(PostUpdate, fragment::mark_children_changed);
    }
}
