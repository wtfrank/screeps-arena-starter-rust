use log::*;
use screeps_arena::{
    constants::{Part, prototypes},
    game,
    prelude::*,
};
use wasm_bindgen::prelude::*;

mod logging;

fn setup() {
    logging::setup_logging(logging::Info);
}

// At startup, the Javascript code in javascript/main.mjs creates an
// instance of this Bot struct, which lasts until the end of the game.
// This means you could add fields to this struct to persist data between
// game ticks, such as creep targets, tasks etc.
#[wasm_bindgen]
pub struct Bot {}

#[wasm_bindgen]
impl Bot {
    // This function is called when the bot object is created in javascript,
    // before the first game tick. The screeps API is therefore generally
    // not available inside this function.
    #[wasm_bindgen(constructor)]
    pub fn initialize() -> Self {
        setup();
        Self {
      // initialise any structures/data in Bot here
    }
    }
    // add wasm_bindgen to any function you would like to expose for call from js
    // to use a reserved name as a function name, use `js_name`:
    #[wasm_bindgen(js_name = loop)]
    pub fn tick(&mut self) {
        let tick = game::utils::get_ticks();

        warn!("hello arena! {}", tick);

        let info = game::arena_info();
        warn!("arena_info: {:?}", info);

        // strategy for spawn and swamp arena, which will conditionally compile in
        // only when this feature is enabled for the crate
        #[cfg(feature = "arena-spawn-and-swamp")]
        {
            let mut enemy_spawn = None;
            let spawns = game::utils::get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
            arn!("spawns {}", spawns.len());
            for spawn in spawns {
                if spawn.my().unwrap_or(false) {
                    spawn.spawn_creep(&[Part::Move, Part::Attack]);
                } else {
                    enemy_spawn = Some(spawn);
                }
            }

            let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
            warn!("creeps {}", creeps.len());
            for creep in creeps {
                if creep.my() {
                    match &enemy_spawn {
                        Some(t) => {
                            creep.move_to(t, None);
                            creep.attack(t);
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
