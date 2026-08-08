"use strict";

import { initSync, Bot } from "./spawn-rush-single-bot";
import wasm_bytes from "./spawn-rush-single-bot_bg.wasm.bin";
const wasm_module = new WebAssembly.Module(wasm_bytes);
initSync({ module: wasm_module });
export * from "./spawn-rush-single-bot";
Error.stackTraceLimit = 100;

// This creates an instance of the rust/wasm Bot struct (defined in
// src/lib.rs) which has lifetime equal to the length of the match.
// When javascript creates this object, the method marked with
// #[wasm_bindgen(constructor)] will be invoked to perform any
// rust-side initialisation.
// This occurs during game setup, before the first game tick
// has started.
let bot = new Bot();

// This provides the function `console.error` that wasm_bindgen sometimes expects to exist,
// especially with type checks in debug mode. An alternative is to have this be `function () {}`
// and let the exception handler log the thrown JS exceptions, but there is some additional
// information that wasm_bindgen only passes here.
//
// There is nothing special about this function and it may also be used by any JS/Rust code as a convenience.
function console_error() {
    const processedArgs = Array.prototype.map.call(arguments, (arg) => {
        if (arg instanceof Error) {
            // On this version of Node, the `stack` property of errors contains
            // the message as well.
            return arg.stack;
        } else {
            return arg;
        }
    }).join(' ');
    console.log("ERROR:", processedArgs);
}

function console_warn() {
    console.log("WARN:", arguments);
}

// Each tick of the game, the screeps game engine calls this javascript loop
// function, which then hands control to rust/wasm code.
function loop () {
  // need to freshly override the fake console object each tick
  console.error = console_error;
  console.warn = console_warn;
  try {
      bot.loop();
  } catch (error) {
      console.error("caught exception:", error);
      // we've already logged the more-descriptive stack trace from rust's panic_hook
      // if for some reason (like wasm init problems) you're not getting output from that
      // and need more information, uncomment the following:
      // if (error.stack) {
      //     console.error("stack trace:", error.stack);
      // }
  }
}

export { loop }
