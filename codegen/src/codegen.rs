use ::data::{Sentence, Preposition};
use macros::get_prep_object;
use tokenizer::emit_error;

// ins dest, src,...
macro_rules! nasm {
    ($ins: expr, $op1: expr, $op2: expr, $op3: expr, $op4: expr) => {
        format!("{} {}, {}, {}, {}", $ins, $op1, $op2, $op3, $op4)
    };
    ($ins: expr, $op1: expr, $op2: expr, $op3: expr) => {
        format!("{} {}, {}, {}", $ins, $op1, $op2, $op3)
    };
    ($ins: expr, $op1: expr, $op2: expr) => {
        format!("{} {}, {}", $ins, $op1, $op2)
    };
    ($ins: expr, $op1: expr) => {
        format!("{} {}", $ins, $op1)
    };
    ($ins: expr) => {
        format!("{}", $ins)
    };
}

fn vt_taking_to(sentence: &Sentence) -> String {
    let _obj = match get_prep_object!(sentence, "obj", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }

    };

    let _to = match get_prep_object!(sentence, "to", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    nasm!(sentence.verb, _to, _obj)
}

fn vi_taking_to(sentence: &Sentence) -> String {
    let _to = match get_prep_object!(sentence, "to", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    nasm!(sentence.verb, _to)
}

fn vt_taking_with(sentence: &Sentence) -> String {
    let _obj = match get_prep_object!(sentence, "obj", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    let _with = match get_prep_object!(sentence, "with", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    nasm!(sentence.verb, _with, _obj)
}

fn vi_taking_with(sentence: &Sentence) -> String {
    let _with = match get_prep_object!(sentence, "with", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    nasm!(sentence.verb, _with)
}

fn vt_taking_by(sentence: &Sentence) -> String {
    let _obj = match get_prep_object!(sentence, "obj", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    let _by = match get_prep_object!(sentence, "by", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    nasm!(sentence.verb, _by, _obj)
}

fn vi_taking_by(sentence: &Sentence) -> String {
    let _by = match get_prep_object!(sentence, "by", sentence.location) {
        (Some(o), loc ) => {
            o
        },
        _ => {
            emit_error!(sentence.location, "this verb takes an object.");
        }
    };

    nasm!(sentence.verb, _by)
}