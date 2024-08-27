pub const VERB: &[&'static str] = &["add", "substract", "multiply", "divide", "move", "jump", "and", "or", "xor", "not", "negate", "shift-right", "shift-left", "call", "compare", "load-effective-address", "push", "pop", "set-byte", "extend-*ax-reg", "return", "halt", "leave", "no-operation", "systemcall"];
pub const PREPOSITION: &[&'static str] = &["to", "from", "by", "as", "with", "if", "for"];
pub const KEYWORD: &[&'static str] = &["single-precision-float", "double-precision-float", "sign-extention", "zero-extention", "signed", "==", "!=", "<", "<=", ">", ">=", "8bit", "16bit", "32bit", "64bit"];
pub const PSEUDO: &[&'static str] = &["define", "globalize", "allocate", "extern"];
pub const REG8: &[&'static str] = &["al", "bl", "cl", "dl", "dil", "sil", "bpl", "spl", "r8b", "r9b", "r10b", "r11b", "r12b", "r13b", "r14b", "r15b"];
pub const REG16: &[&'static str] = &["ax", "bx", "cx", "dx", "di", "si", "bp", "sp", "r8w", "r9w", "r10w", "r11w", "r12w", "r13w", "r14w", "r15w"];
pub const REG32: &[&'static str] = &["eax", "ebx", "ecx", "edx", "edi", "esi", "ebp", "esp", "r8d", "r9d", "r10d", "r11d", "r12d", "r13d", "r14d", "r15d"];
pub const REG64: &[&'static str] = &["rax", "rbx", "rcx", "rdx", "rdi", "rsi", "rbp", "rsp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15", "rip"];

