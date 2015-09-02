
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

    world.update();
}
