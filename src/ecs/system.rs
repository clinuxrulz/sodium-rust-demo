use ecs::EcsContext;
use ecs::EcsEventContext;

pub trait System<ECTX: EcsContext, EVCTX: EcsEventContext<ECTX>> {
    fn system_name() -> &'static str;
    fn configure(&mut self, eventCtx: &mut EVCTX);
}
