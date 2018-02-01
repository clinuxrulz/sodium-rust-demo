use math::Vec3;

#[derive(Copy, Clone)]
pub struct Plane3D<A> {
    pub normal: Vec3<A>,
    pub d: A
}
