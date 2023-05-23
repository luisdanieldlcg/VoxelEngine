use bevy_ecs::{prelude::Component, system::Query};
use vek::Vec3;

#[derive(Component)]
pub struct RigidBody;

#[derive(Component)]
pub struct BoxCollider;

#[derive(Component)]
pub struct Transform {
    pub pos: Vec3<i32>,
}

fn update_physics(q: Query<(&RigidBody, &BoxCollider, &Transform)>, dt: f32) {
    for (rbody, collider, transform) in q.iter() {}
}
