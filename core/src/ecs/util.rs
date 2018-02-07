use ecs::components::Axes3DComponent;
use ecs::EcsContext;
use ecs::Entity;
use ecs::IsComponent;
use math::Axes3D;

pub fn ecs_get_axes3d<ECTX:EcsContext>(ectx: &mut ECTX, entity: &Entity) -> Axes3D<f64> {
    ectx.get_component(entity, Axes3DComponent::component())
        .map(|x| x.axes)
        .unwrap_or_else(|| Axes3D::identity())
}
