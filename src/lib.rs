use bevy::prelude::*;

mod computations;
mod effect_cell;
mod insert_when;

pub use computations::{ComputationOf, Computations};
pub use insert_when::{InsertWhen, insert_when};

use crate::effect_cell::update_effects;

pub struct ThoriumPlugin;

impl Plugin for ThoriumPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_effects);
        // app.add_systems(PostUpdate, fragment::mark_children_changed);
    }
}
