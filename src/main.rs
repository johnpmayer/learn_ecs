
#[macro_use]
extern crate ecs;

use ecs::{BuildData, World};
use ecs::entity::{EntityIter};
use ecs::system::{Process, System};
use ecs::system::entity::{EntityProcess, EntitySystem};
use ecs::world::{DataHelper};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

components! {
    struct MyComponents {
        #[hot] position: Position,
        #[hot] velocity: Velocity
    }
}

pub struct PrintMessage(pub String);

impl System for PrintMessage { type Components = MyComponents; type Services = (); fn is_active(&self) -> bool { false } }

impl Process for PrintMessage {
    fn process(&mut self, _: &mut DataHelper<MyComponents, ()>) {
        println!("{}", &self.0);
    }
}

pub struct MotionProcess;

impl System for MotionProcess { type Components = MyComponents; type Services = (); }

impl EntityProcess for MotionProcess {
    fn process(&mut self, entities: EntityIter<MyComponents>, data: &mut
               DataHelper<MyComponents,()>) {
        // Note the difference between this and a regular Process. Gets an additional iterator of
        // entities
        for e in entities {
            let mut position = data.position[e];
            let     velocity = data.velocity[e];
            position.x += velocity.dx;
            position.y += velocity.dy;
            data.position[e] = position;
        }
    }
}

systems! {
    struct MySystems<MyComponents, ()> {
        print_msg: PrintMessage = PrintMessage("Hello, PrintMessage System!".to_string()),
        motion: EntitySystem<MotionProcess> = EntitySystem::new(
            MotionProcess,
            aspect!(<MyComponents> all: [position, velocity])
            )
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
        data.velocity.add(&entity, Velocity { dx : 0.1, dy : 0.2 });
    });
    println!("{:?}", entity);

    // Manually run the system process
    process!(world, print_msg); 

    // Updating a system
    world.systems.print_msg.0 = "Hello, Updated PrintMessage!".to_string();
    process!(world, print_msg); 

    for _ in 0..5 {
        world.with_entity_data(&entity, |e, data| {
            println!("Position: {:?} {:?}", data.position[e].x, data.position[e].y);
        });
        world.update();

    }

}
