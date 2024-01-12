#![feature(unboxed_closures)]
use std::sync::Arc;

use crate::{Interface, Proxy};

type Output = dyn FnOnce() -> Args;
type Args = Dyn;
type Dyn = Proxy;
type Error = Arc<dyn Interface>;

impl FnOnce() -> () for Dyn {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output {
        todo!()
    }
}