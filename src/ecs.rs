use becs::World;

pub struct Position(pub f32, pub f32);
pub struct Speed(pub f32);
pub struct Movement(pub f32, pub f32);

pub(crate) fn configure_ecs() -> World {
    let mut world = World::new();
    let entity = world.new_entity();

    world.add_component_to_entity(entity, Position(0.0, 0.0));
    world.add_component_to_entity(entity, Speed(50.0));
    world.add_component_to_entity(entity, Movement(1.5, 1.0));
    world
}

#[profiling::function]
pub fn movement_system(world: &mut World, delta: f32) {
    let mut positions = world.borrow_component_vec_mut::<Position>().unwrap();
    let movements = world.borrow_component_vec::<Movement>().unwrap();
    let speeds = world.borrow_component_vec::<Speed>().unwrap();

    // Update position
    let zip = positions
        .iter_mut()
        .zip(speeds.iter())
        .zip(movements.iter());
    for (position, speed, movement) in zip.filter_map(|((position, speed), movement)| {
        Some((position.as_mut()?, speed.as_ref()?, movement.as_ref()?))
    }) {
        position.0 += movement.0 * speed.0 * delta;
        position.1 += movement.1 * speed.0 * delta;
    }
}
