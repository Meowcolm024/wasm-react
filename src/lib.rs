mod bf;

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
pub fn run_bf(prog: String, input: String, cells: usize, fuel: usize) -> String {
    let prog = prog.bytes().collect::<Vec<u8>>();
    let input = input.bytes().collect::<Vec<u8>>();
    let res = bf::eval(prog, input, bf::Config::new(cells, fuel));
    match res {
        Ok(v) => v.into_iter().map(|x| x as char).collect(),
        Err(msg) => msg,
    }
}
