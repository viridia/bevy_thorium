use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    scene2::Scene,
};

use crate::{
    ComputationOf,
    effect_cell::{AnyEffect, EffectCell},
};

pub struct InsertWhenEffect<B: Bundle, FactoryFn: Fn() -> B + Clone> {
    state: bool,
    test_sys: SystemId<(), bool>,
    factory: FactoryFn,
}

impl<B: Bundle, FactoryFn: Fn() -> B + Clone> AnyEffect for InsertWhenEffect<B, FactoryFn> {
    fn update(&mut self, world: &mut World, entity: Entity) {
        let Some(owner) = world.get::<ComputationOf>(entity) else {
            return;
        };
        let owner = owner.get();
        // Run the condition and see if the result changed.
        let test = world.run_system(self.test_sys);
        if let Ok(test) = test {
            if self.state != test {
                if test {
                    world.commands().entity(owner).insert((self.factory)());
                } else {
                    world.commands().entity(owner).remove::<B>();
                }
                self.state = test;
            }
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.test_sys);
    }
}

pub struct InsertWhen<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Clone + Send + Sync + 'static,
    B: Bundle,
    FactoryFn: Fn() -> B + Clone + Send + Sync + 'static,
> {
    test_fn: TestFn,
    factory: FactoryFn,
    marker: std::marker::PhantomData<M>,
}

impl<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Clone + Send + Sync + 'static,
    B: Bundle,
    FactoryFn: Fn() -> B + Clone + Send + Sync + 'static,
> InsertWhen<M, TestFn, B, FactoryFn>
{
    pub fn new(test_fn: TestFn, factory: FactoryFn) -> Self {
        Self {
            test_fn,
            factory,
            marker: std::marker::PhantomData,
        }
    }
}

impl<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Clone + Send + Sync + 'static,
    B: Bundle,
    FactoryFn: Fn() -> B + Clone + Send + Sync + 'static,
> Template for InsertWhen<M, TestFn, B, FactoryFn>
{
    type Output = ();

    fn build(&mut self, parent: &mut EntityWorldMut) -> Result<Self::Output> {
        let test_sys = parent.world_scope(|world| world.register_system(self.test_fn.clone()));
        parent.with_related::<ComputationOf>(EffectCell::new(InsertWhenEffect {
            state: false,
            test_sys,
            factory: self.factory.clone(),
        }));
        Ok(())
    }
}

impl<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Clone + Send + Sync + 'static,
    B: Bundle,
    FactoryFn: Fn() -> B + Clone + Send + Sync + 'static,
> Scene for InsertWhen<M, TestFn, B, FactoryFn>
{
    fn patch(
        &self,
        _assets: &AssetServer,
        _patches: &Assets<bevy::scene2::ScenePatch>,
        scene: &mut bevy::scene2::ResolvedScene,
    ) {
        scene.push_template(InsertWhen {
            test_fn: self.test_fn.clone(),
            factory: self.factory.clone(),
            marker: std::marker::PhantomData,
        });
    }
}

pub fn insert_when<
    M: Send + Sync + 'static,
    TestFn: IntoSystem<(), bool, M> + Clone + Send + Sync + 'static,
    B: Bundle,
    FactoryFn: Fn() -> B + Clone + Send + Sync + 'static,
>(
    test_fn: TestFn,
    factory: FactoryFn,
) -> impl Scene {
    InsertWhen {
        test_fn,
        factory,
        marker: std::marker::PhantomData,
    }
}
