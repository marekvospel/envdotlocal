mod parser;
mod utils;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::parse_dotenv;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(s: &str) -> Option<String> {
    parse_dotenv(s)
}
