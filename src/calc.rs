use bevy::{
    ecs::{system::SystemId, world::DeferredWorld},
    prelude::*,
    scene2::Scene,
};

use crate::{
    computations::ComputationOf,
    effect_cell::{AnyEffect, EffectCell},
};

pub struct CalcEffect<P, M, EffectFn: Fn(&mut EntityWorldMut, P)> {
    deps: Option<P>,
    deps_sys: SystemId<(), P>,
    effect_fn: EffectFn,
    marker: std::marker::PhantomData<M>,
}

impl<P: 'static + PartialEq + Send + Sync + Clone, M, EffectFn: Fn(&mut EntityWorldMut, P)>
    AnyEffect for CalcEffect<P, M, EffectFn>
{
    fn update(&mut self, world: &mut World, entity: Entity) {
        let Some(owner) = world.get::<ComputationOf>(entity) else {
            return;
        };
        let owner_id = owner.get();
        // Run the dependencies and see if the result changed.
        let deps = world.run_system(self.deps_sys).ok();
        if deps.is_some() && deps != self.deps {
            self.deps = deps.clone();
            // Run the effect
            (self.effect_fn)(&mut world.entity_mut(owner_id), deps.unwrap());
        }
    }

    fn cleanup(&self, world: &mut DeferredWorld, _entity: Entity) {
        world.commands().unregister_system(self.deps_sys);
    }
}

#[derive(Clone)]
pub struct Calc<
    P: PartialEq + Clone + Send + Sync + 'static,
    M: Send + Sync + 'static,
    DepsFn: IntoSystem<(), P, M> + Clone + 'static,
    EffectFn: Fn(&mut EntityWorldMut, P) + Clone + Send + Sync + 'static,
> {
    deps_fn: DepsFn,
    effect_fn: EffectFn,
    marker: std::marker::PhantomData<(P, M)>,
}

impl<
    P: PartialEq + Clone + Send + Sync + 'static,
    M: Send + Sync + 'static,
    DepsFn: IntoSystem<(), P, M> + Clone + Send + Sync + 'static,
    EffectFn: Fn(&mut EntityWorldMut, P) + Clone + Send + Sync + 'static,
> Calc<P, M, DepsFn, EffectFn>
{
    pub fn new(deps_fn: DepsFn, effect_fn: EffectFn) -> Self {
        Self {
            deps_fn,
            effect_fn,
            marker: std::marker::PhantomData,
        }
    }
}

impl<
    P: PartialEq + Clone + Send + Sync + 'static,
    M: Send + Sync + 'static,
    DepsFn: IntoSystem<(), P, M> + Clone + Send + Sync + 'static,
    EffectFn: Fn(&mut EntityWorldMut, P) + Clone + Send + Sync + 'static,
> Template for Calc<P, M, DepsFn, EffectFn>
{
    type Output = ();

    fn build(&mut self, parent: &mut EntityWorldMut) -> Result<Self::Output> {
        let deps_sys = parent.world_scope(|world| world.register_system(self.deps_fn.clone()));
        parent.with_related::<ComputationOf>(EffectCell::new(CalcEffect {
            deps: None,
            deps_sys,
            effect_fn: self.effect_fn.clone(),
            marker: std::marker::PhantomData::<M>,
        }));
        Ok(())
    }
}

impl<
    P: PartialEq + Clone + Send + Sync + 'static,
    M: Send + Sync + 'static,
    DepsFn: IntoSystem<(), P, M> + Clone + Send + Sync + 'static,
    EffectFn: Fn(&mut EntityWorldMut, P) + Clone + Send + Sync + 'static,
> Scene for Calc<P, M, DepsFn, EffectFn>
{
    fn patch(
        &self,
        _assets: &AssetServer,
        _patches: &Assets<bevy::scene2::ScenePatch>,
        scene: &mut bevy::scene2::ResolvedScene,
    ) {
        scene.push_template(Calc {
            deps_fn: self.deps_fn.clone(),
            effect_fn: self.effect_fn.clone(),
            marker: std::marker::PhantomData,
        });
    }
}

pub fn calc<
    P: PartialEq + Clone + Send + Sync + 'static,
    M: Send + Sync + 'static,
    DepsFn: IntoSystem<(), P, M> + Clone + Send + Sync + 'static,
    EffectFn: Fn(&mut EntityWorldMut, P) + Clone + Send + Sync + 'static,
>(
    deps_fn: DepsFn,
    effect_fn: EffectFn,
) -> impl Scene {
    Calc {
        deps_fn,
        effect_fn,
        marker: std::marker::PhantomData,
    }
}
