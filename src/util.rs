pub struct Config {
    pub cells: usize,
    pub fuel: usize,
}

impl Config {
    pub fn new(cells: usize, fuel: usize) -> Self {
        Self { cells, fuel }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cells: 128,
            fuel: 4096,
        }
    }
}

pub trait RW {
    fn read(&mut self) -> Option<u8>;
    fn write(&mut self, b: u8);
}

pub struct IOBuf {
    pub input: Vec<u8>,
    pub output: Vec<u8>,
}

impl IOBuf {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input: input,
            output: vec![],
        }
    }

    pub fn reset(&mut self, input: Vec<u8>) {
        self.input = input;
        self.output.clear();
    }
}

impl RW for IOBuf {
    fn read(&mut self) -> Option<u8> {
        self.input.pop()
    }

    fn write(&mut self, b: u8) {
        self.output.push(b)
    }
}
