use crate::util::{Config, RW};

pub struct Machine {
    st: Vec<usize>, // stack
    mem: Vec<u8>,   // memory
    mp: usize,      // memory pointer
    ip: usize,      // instruction pointer
    fuel: usize,    // max steps
}

impl Machine {
    pub fn new(config: Config) -> Self {
        Self {
            st: Vec::new(),
            mem: vec![0; config.cells],
            mp: 0,
            ip: 0,
            fuel: config.fuel,
        }
    }

    fn reset(&mut self) {
        self.st.clear();
        self.mem.iter_mut().for_each(|x| *x = 0);
        self.mp = 0;
        self.ip = 0;
    }

    /** execute bf program, read and write via io  */
    pub fn exec(&mut self, prog: Vec<u8>, io: &mut impl RW) -> Result<(), &'static str> {
        self.reset();
        let mut fuel = 0;
        while self.ip < prog.len() {
            if fuel == self.fuel {
                return Err("max steps reached");
            } else {
                fuel += 1;
            }
            match prog[self.ip] {
                b'+' => self.mem[self.mp] = self.mem[self.mp].overflowing_add(1).0,
                b'-' => self.mem[self.mp] = self.mem[self.mp].overflowing_sub(1).0,
                b'[' => self.st.push(self.ip),
                b']' => {
                    if self.mem[self.mp] == 0 {
                        self.st.pop();
                    } else {
                        match self.st.last() {
                            Some(i) => self.ip = *i,
                            None => return Err("jump error"),
                        }
                        continue;
                    }
                }
                b'>' => {
                    if self.mp == self.mem.len() - 1 {
                        return Err("cannot move beyond right bound");
                    } else {
                        self.mp += 1
                    }
                }
                b'<' => {
                    if self.mp == 0 {
                        return Err("cannot move beyond left bound");
                    } else {
                        self.mp -= 1
                    }
                }
                b'.' => io.write(self.mem[self.mp]),
                b',' => {
                    self.mem[self.mp] = match io.read() {
                        Some(b) => b,
                        None => return Err("not input"),
                    }
                }
                _ => (),
            }
            self.ip += 1;
        }
        Ok(())
    }
}

mod test {
    #[test]
    fn ex() {
        let prog = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let prog = prog.as_bytes().into_iter().map(|x| *x).collect::<Vec<u8>>();
        let mut buf = crate::util::IOBuf::new(vec![]);
        let mut machine = super::Machine::new(super::Config::default());
        machine.exec(prog, &mut buf).unwrap();
        let res: String = buf.output.into_iter().map(|x| x as char).collect();
        println!("res = {:?}", res);
    }

    #[test]
    fn er() {
        let mut machine = super::Machine::new(super::Config::new(4, 1000));
        let mut buf = crate::util::IOBuf::new(vec![]);
        let lb = machine.exec(vec![b'<'], &mut buf);
        assert!(lb.is_err());
        let rb = machine.exec(vec![b'>'; 5], &mut buf);
        assert!(rb.is_err());
        let ip = machine.exec(vec![b','], &mut buf);
        assert!(ip.is_err());
    }
}
