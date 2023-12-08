pub mod bf;
pub mod util;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_bf(prog: String, input: String, cells: usize, fuel: usize) -> String {
    let mut io = util::IOBuf::new(input.bytes().rev().collect());
    let mut machine = bf::Machine::new(util::Config::new(cells, fuel));
    let res = machine.exec(prog.into(), &mut io);
    let output = String::from_utf8_lossy(&io.output).into();
    if let Err(msg) = res {
        format!("{} [Error: {}]", output, msg)
    } else {
        output
    }
}
