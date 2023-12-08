mod bf;

use bf::eval;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn run_bf(prog: String) -> String {
    let prog = prog.bytes().collect::<Vec<u8>>();
    let res = eval(prog, vec![]);
    res.into_iter().map(|x| x as char).collect()
}
