use bevy_ecs::{prelude::Component, system::Query};
use vek::{Vec3, Vec2};

#[derive(Component)]
pub struct RigidBody;

#[derive(Component)]
pub struct BoundingBox {
    pub size: Vec2<i32>
}

#[derive(Component)]
pub struct Transform {
    pub pos: Vec3<i32>,
}

fn update_physics(q: Query<(&RigidBody, &BoundingBox, &Transform)>, dt: f32) {
    for (rbody, collider, transform) in q.iter() {}
}
