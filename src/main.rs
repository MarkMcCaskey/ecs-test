#[macro_use]
extern crate log;
extern crate fern;
extern crate nalgebra;
extern crate nphysics2d;
extern crate sdl2;
extern crate specs;

use specs::prelude::*;

pub mod components;
pub mod lexer;
pub mod parser;
pub mod render;
use components::*;
use nalgebra::{distance_squared, Vector2};
use parser::{parse_string, Ast};
use render::*;

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (p, v) in (&mut pos, &vel).join() {
            p.0 += v.0;
        }
    }
}

struct GravitySys;

impl<'a> System<'a> for GravitySys {
    type SystemData = (
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Mass>,
    );

    fn run(&mut self, (pos, mut vel, mass): Self::SystemData) {
        for (p, v) in (&pos, &mut vel).join() {
            for (m_p, mass) in (&pos, &mass).join() {
                let sq_dist = distance_squared(&p.0, &m_p.0);
                let new_vel_vec: Vector2<f32> = (m_p.0 - p.0) * (mass.0 / sq_dist);
                v.0 += new_vel_vec;
            }
        }
    }
}

struct CommetCreator;

impl<'a> System<'a> for CommetCreator {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, AstCommands>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
    );

    fn run(&mut self, (entities, mut commands, mut pos, mut vel): Self::SystemData) {
        while let Some(command) = commands.0.pop() {
            match command {
                Ast::SpawnCommand {
                    name: _name,
                    location,
                } => {
                    let commet = entities.create();

                    pos.insert(commet, Pos::new(location.x, location.y))
                        .unwrap();
                    vel.insert(commet, Vel::new(0., 0.)).unwrap();
                }
                _ => println!("Delete not implemented"),
            }
        }
    }
}

struct AstCommands(Vec<Ast>);

fn main() {
    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<Mass>();

    world
        .create_entity()
        .with(Pos::new(360., 270.))
        .with(Mass(20.))
        .build();

    world.add_resource(AstCommands(Vec::new()));

    let mut dispatcher = DispatcherBuilder::new()
        .with(CommetCreator, "commet_creator", &[])
        .with(GravitySys, "gravity", &["commet_creator"])
        .with(SysA, "sys_a", &["gravity"])
        .build();

    let mut render_system = RenderSystem::new();

    world
        .write_resource::<AstCommands>()
        .0
        .push(parse_string("spawn commet at 123, 123".to_owned()).unwrap());

    debug!("Systems initialized, beginning loop");
    loop {
        dispatcher.dispatch(&world.res);
        render_system.handle_events(&mut world);
        render_system.clear();
        for entity in world.read_storage::<Pos>().join() {
            render_system.draw_box(entity.0);
        }
        render_system.draw();
    }
}
