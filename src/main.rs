
#[macro_use]
extern crate ecs;

use ecs::World;

struct Shape;

components! {
    struct MyComponents;
}

systems! {
    struct MySystems<MyComponents, ()>;
}

fn main() {
    println!("Hello, world!");
    let mut world = World::<MySystems>::new();

    let entity = world.create_entity(());
    println!("{:?}", entity);
}
