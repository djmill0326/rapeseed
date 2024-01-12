use crate::into::input_stream;

mod input_stream;

fn main() -> input_stream::Dyn {
    dbg!("[alloc] loaded...");
    input_stream::Dyn()
}

pub trait Into {}