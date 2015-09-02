
#[macro_use]
extern crate ecs;

use ecs::{BuildData,World};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

components! {
    struct MyComponents {
        #[hot] position: Position,
        #[cold] respawn: Position
    }
}

systems! {
    struct MySystems<MyComponents, ()>;
}

fn main() {
    println!("Hello, world!");
    let mut world = World::<MySystems>::new();

    let entity_rm = world.create_entity(());
    println!("{:?}", entity_rm);
    world.remove_entity(entity_rm);

    let entity = world.create_entity(|entity: BuildData<MyComponents>, data: &mut MyComponents| {
        data.position.add(&entity, Position { x : 0.0, y : 0.0 });
        data.respawn.add(&entity, Position { x : 0.0, y : 0.0 });
    });
    assert!(entity_rm != entity);
    println!("{:?}", entity);
}
