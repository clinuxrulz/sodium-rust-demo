use ecs::EcsContext;
use ecs::EcsEventContext;

pub trait System<ENV, ECTX: EcsContext, EVCTX: EcsEventContext<ENV, ECTX>> {
    fn system_name() -> &'static str;
    fn configure(&mut self, event_ctx: &mut EVCTX);
}
