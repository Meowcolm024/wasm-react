pub struct Config {
    cells: usize,
    fuel: usize,
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
            fuel: 65535,
        }
    }
}

pub fn eval(prog: Vec<u8>, mut input: Vec<u8>, mut config: Config) -> Result<Vec<u8>, String> {
    let mut ip = 0; // instruction pointer
    let mut st: Vec<usize> = Vec::new(); // stack
    let mut res: Vec<u8> = Vec::new(); // output buffer
    let mut mem: Vec<u8> = vec![0; config.cells]; // memory
    let mut mp = 0; // memory pointer
    input.reverse();
    while ip < prog.len() {
        if config.fuel == 0 {
            return Err(String::from("Error: max steps reached"));
        } else {
            config.fuel -= 1;
        }
        match prog[ip] {
            b'+' => mem[mp] = mem[mp].overflowing_add(1).0,
            b'-' => mem[mp] = mem[mp].overflowing_sub(1).0,
            b'[' => st.push(ip),
            b']' => {
                if mem[mp] == 0 {
                    st.pop();
                } else {
                    match st.last() {
                        Some(i) => ip = *i,
                        None => return Err(String::from("Error: jump error")),
                    }
                    continue;
                }
            }
            b'>' => {
                if mp == config.cells - 1 {
                    return Err(String::from("Error: cannot move beyond right bound"));
                } else {
                    mp += 1
                }
            }
            b'<' => {
                if mp == 0 {
                    return Err(String::from("Error: cannot move beyond left bound"));
                } else {
                    mp -= 1
                }
            }
            b'.' => res.push(mem[mp]),
            b',' => {
                mem[mp] = match input.pop() {
                    Some(b) => b,
                    None => return Err(String::from("Error: not input")),
                }
            }
            _ => (),
        }
        ip += 1;
    }
    Ok(res)
}

mod test {

    #[test]
    fn ex() {
        let prog = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let prog = prog.as_bytes().into_iter().map(|x| *x).collect::<Vec<u8>>();
        let res = super::eval(prog, vec![], super::Config::default()).unwrap();
        let res: String = res.into_iter().map(|x| x as char).collect();
        println!("res = {:?}", res);
    }

    #[test]
    fn er() {
        let lb = super::eval(vec![b'<'], vec![], super::Config::default());
        assert!(lb.is_err());
        let rb = super::eval(vec![b'>'; 5], vec![], super::Config::new(4, 1000));
        assert!(rb.is_err());
        let ip = super::eval(vec![b','], vec![], super::Config::default());
        assert!(ip.is_err());
    }
}
