fn main() {
    println!("Hello, world!");
}

fn read_ins<'a>(ins: &'a str) -> (Vec<&'a str>, ()) {
    let mut args = Vec::new();
    let mut i = 0;
    loop {
        
        match ins.chars().nth(i) {
            None => {
                break;
            },
            Some('0') => {
                // todo!()
                i += 1;
            },
            Some('1') => {
                // todo!()
                i += 1;
            },
            Some(':') | Some(' ') => {
                // todo!()
                i += 1;
            },
            Some('B') | Some('R') | Some('X') | Some('W') | Some('w') | Some('s') => {
                // todo!()
                args.push(&ins[i..(i + 1)]);
                i += 1;
            },
            _ => {
                let end = read_args(&ins, i);
                args.push(&ins[i..end]);
                i += end;
            }
        }
    }
    (args, ())
}

fn read_args<'a>(args: &'a str, start: usize) -> usize {
    let mut i = 0;
    while args.chars().nth(start + i).is_some_and(|c| c == ' ' || c == ':'){
        i += 1;
    }
    todo!()
}
