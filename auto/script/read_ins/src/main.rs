fn main() {
    println!("{:#?}", read_ins("0100 100B 1111 : 1011 1010 : 11 100 qwordreg: imm8 data"));
}

fn read_ins<'a>(ins: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut args = Vec::new();
    let mut bits = Vec::new();
    let mut i = 0;
    loop {
        
        match ins.chars().nth(i) {
            None => {
                break;
            },
            Some('0') | Some('1') => {
                // todo!()
                bits.push(&ins[i..(i + 1)]);
                i += 1;
            },
            Some(':') | Some(' ') => {
                // todo!()
                i += 1;
            },
            Some('B') | Some('R') | Some('X') | Some('W') | Some('w') | Some('s') => {
                // todo!()
                let arg = &ins[i..(i + 1)];
                args.push(arg);
                bits.push(arg);
                i += 1;
            },
            _ => {
                let end = read_args(&ins, i);
                let arg = &ins[i..(i + end)];
                args.push(arg);
                bits.push(arg);
                i += end;
            }
        }
    }
    (args, bits)
}

fn read_args<'a>(args: &'a str, start: usize) -> usize {
    let mut i = 0;
    while args.chars().nth(start + i).is_some_and(|c| c != ' ' && c != ':'){
        i += 1;
    }
    i
}
