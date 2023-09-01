use crate::machine::types::Physics;

pub fn physics_system(mut query: Query<&mut Position, &mut Velocity, &mut Acceleration, &Physics>) {
    for (mut pos, mut vel, mut acc, physics) in query.iter_mut() {
        match physics {
            Physics::NotMoving => (),
        }
    }
}
