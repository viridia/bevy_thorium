use bevy::ecs::{
    system::{IntoSystem, SystemId},
    world::World,
};

pub trait Signal<T, M = ()> {
    fn into_system_id(self, world: &mut World) -> SystemId<(), T>;
}

impl<T> Signal<T, ()> for SystemId<(), T> {
    fn into_system_id(self, world: &mut World) -> SystemId<(), T> {
        self
    }
}

// impl<T: 'static, M, S: IntoSystem<(), T, M> + 'static> Signal<T, M> for S {
//     fn into_system_id(self, world: &mut World) -> SystemId<(), T> {
//         world.register_system(self)
//     }
// }
