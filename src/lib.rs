#[cfg(target_arch = "wasm32")]
pub extern crate screeps_arena_wasm as screeps_arena;
#[cfg(target_arch = "wasm32")]
pub extern crate wasm_bindgen_wasm as wasm_bindgen;

#[cfg(not(target_arch = "wasm32"))]
pub extern crate screeps_arena_mock as screeps_arena;
#[cfg(not(target_arch = "wasm32"))]
pub extern crate wasm_bindgen_mock as wasm_bindgen;

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

        let info = game::arena_info();
        warn!("arena_info: {:?}", info);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bot_initialize() -> *mut Bot {
    Box::into_raw(Box::new(Bot::initialize()))
}

#[cfg(not(target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bot_tick(bot: *mut Bot) {
    unsafe {
        let bot = &mut *bot;
        bot.tick();
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn bot_free(bot: *mut Bot) {
    unsafe {
        if !bot.is_null() {
            let _ = Box::from_raw(bot);
        }
    }
}
