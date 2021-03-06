extern crate sfml;
extern crate rand;
extern crate num;
extern crate nalgebra as na;
extern crate rustc_serialize;
extern crate poglgame;
extern crate recs;

use std::env::args;
use std::collections::{HashSet, HashMap};
use std::iter::FromIterator;
use std::cell::Cell;

use old_engine::launch;
use gameplay::GameplayScreen;
use generator::generate_default;

mod util;
mod generator;
mod graph;
mod search;
mod solver;
mod old_engine;
mod collision;
mod animation;
mod entities;
mod gameplay;

mod utils;
mod components;
mod rect;
mod physics;
mod gfx;
mod world;
mod screens;

mod test_gen;
mod test_search;
mod test_json;
mod test_new;
mod test_dungeon;

use utils::float;
use screens::GameplayScreen as NewGameplayScreen;

fn main() {

    let use_new = Cell::new(false);
    let run_game = Cell::new(true);
    let mut fn_map: HashMap<String, Box<Fn()>> = HashMap::new();
    fn_map.insert("--gen".into(),
            Box::new( || { test_gen::main(); run_game.set(false); } ));
    fn_map.insert("--search".into(),
            Box::new( || { test_search::main(); run_game.set(false); } ));
    fn_map.insert("--json".into(),
            Box::new( || { test_json::main(); run_game.set(false); } ));
    fn_map.insert("--dungeon".into(),
            Box::new( || { test_dungeon::main(); run_game.set(false); } ));
    fn_map.insert("--new".into(),
            Box::new( || { use_new.set(true); } ));

    let unique_args: HashSet<String> = HashSet::from_iter(args());

    for arg in unique_args.iter() {
        fn_map.remove(arg).map(|ref mut f| f());
    }

    if !run_game.get() { return; }

    if use_new.get() {
        let w = 800;
        let h = 800;
        let scr = NewGameplayScreen::new(w as float, h as float);
        poglgame::launch(scr, "Rusty Rogue", w, h);
    } else {
        launch(GameplayScreen::new( &generate_default( 123 ) ), "Rusty Rogue",
                800, 600);
    }
}