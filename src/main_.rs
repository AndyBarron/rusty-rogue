extern crate rsfml;
extern crate rand;
extern crate collections;
extern crate native;
extern crate sync;

use engine::launch;
use gameplay::GameplayScreen;
use generator::generate_default;

mod util;
mod generator;
mod graph;
mod search;
mod solver;
mod engine;
mod collision;
mod animation;
mod entities;
mod gameplay;

#[start]
fn start(argc: int, argv: **u8) -> int { native::start(argc, argv, main) }

fn main() {
	launch(~GameplayScreen::new( &generate_default( 123 ) ),"Rusty Rogue",800,600);
}