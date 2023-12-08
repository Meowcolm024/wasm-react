pub fn eval(prog: Vec<u8>, mut input: Vec<u8>) -> Vec<u8> {
    let mut ip = 0;
    let mut st: Vec<usize> = Vec::new();
    let mut res: Vec<u8> = Vec::new();
    let mut mem: Vec<u8> = vec![0; 32];
    let mut mp = 0;
    while ip < prog.len() {
        match prog[ip] {
            b'+' => mem[mp] = mem[mp].overflowing_add(1).0,
            b'-' => mem[mp] = mem[mp].overflowing_sub(1).0,
            b'[' => st.push(ip),
            b']' => {
                if mem[mp] == 0 {
                    st.pop();
                } else {
                    ip = *st.last().unwrap();
                    continue;
                }
            }
            b'>' => mp += 1,
            b'<' => mp -= 1,
            b'.' => res.push(mem[mp]),
            b',' => mem[mp] = input.pop().unwrap(),
            _ => (),
        }
        ip += 1;
    }
    res
}

mod test {
    use super::eval;

    #[test]
    fn ex() {
        let prog = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
        let prog = prog.as_bytes().into_iter().map(|x| *x).collect::<Vec<u8>>();
        let res = eval(prog, vec![]);
        let res: String = res.into_iter().map(|x| x as char).collect();
        println!("res = {:?}", res);
    }
}
