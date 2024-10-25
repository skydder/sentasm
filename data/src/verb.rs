#[derive(Clone, Copy, Debug)]
pub struct Verb<'a>(pub &'a str);

impl<'a> std::fmt::Display for Verb<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self("add") => write!(f, "add"),
            Self("substract") => write!(f, "sub"),
            Self("multiply") => write!(f, "mul"),
            Self("divide") => write!(f, "div"),
            Self("move") => write!(f, "mov"),
            Self("jump") => write!(f, "jmp"),
            Self("and") => write!(f, "and"),
            Self("or") => write!(f, "or"),
            Self("xor") => write!(f, "xor"),
            Self("not") => write!(f, "not"),
            Self("negate") => write!(f, "neg"),
            Self("shift-rigt") => write!(f, "shr"),
            Self("shift-left") => write!(f, "shl"),
            Self("call") => write!(f, "call"),
            Self("compare") => write!(f, "cmp"),
            Self("return") => write!(f, "ret"),
            Self("leave") => write!(f, "leave"),
            Self("no-operation") => write!(f, "nop"),
            Self("systemcall") => write!(f, "syscall"),
            Self("halt") => write!(f, "hlt"),
            Self("load-effective-address") => write!(f, "lea"),
            Self("define") => write!(f, "data"),
            Self("push") => write!(f, "push"),
            Self("pop") => write!(f, "pop"),
            Self("globalize") => write!(f, "global"),
            Self("allocate") => write!(f, "bss"),
            Self("set-byte") => write!(f, "set"),
            Self("extend-*ax-reg") => write!(f, "cqo, etc"),
            Self("extern") => write!(f, "extern"),
            _ => write!(f, "Not Implemented"),
        }
    }
}