
#[macro_use]
extern crate ecs;

use ecs::{BuildData, World};
use ecs::system::{Process, System};
use ecs::world::{DataHelper};

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

pub struct PrintMessage(pub String);

impl System for PrintMessage {
    type Components = MyComponents;
    type Services = ();
    fn is_active(&self) -> bool { false }
}

impl Process for PrintMessage {
    fn process(&mut self, _: &mut DataHelper<MyComponents, ()>) {
        println!("{}", &self.0);
    }
}

systems! {
    struct MySystems<MyComponents, ()> {
        print_msg: PrintMessage = PrintMessage("Hello, PrintMessage System!".to_string())
    }
}

fn main() {
    println!("Hello, ecs!");

    // Create a new esc world
    let mut world = World::<MySystems>::new();

    // Testing that entities are unique
    let entity1 = world.create_entity(());
    println!("{:?}", entity1);
    world.remove_entity(entity1);
    let entity2 = world.create_entity(());
    assert!(entity1 != entity2);
    println!("{:?}", entity2);
    world.remove_entity(entity2);

    // Creating an entity with initialized components
    let entity = world.create_entity(|entity: BuildData<MyComponents>, data: &mut MyComponents| {
        data.position.add(&entity, Position { x : 0.0, y : 0.0 });
        data.respawn.add(&entity, Position { x : 0.0, y : 0.0 });
    });
    println!("{:?}", entity);

    // Manually run the system process
    process!(world, print_msg); 

    // Updating a system
    world.systems.print_msg.0 = "Hello, Updated PrintMessage!".to_string();
    process!(world, print_msg); 
}
