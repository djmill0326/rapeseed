/* #!/bin/curl http://ehpt:666/haptloader/v1 >> /bin/fish */

use std::collections::VecDeque;
use std::f32::consts::E;
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicIsize};
use rand::random;

trait Scanner {
    fn scan(self) -> Tree;
}

impl Scanner for Tree {
    fn scan(self) -> Tree {
        println!("[bootloader/war] <{:?},{:?}>", self.mux(self).0, self.mux(self).0);
        self
    }
}

fn adapter(c: [usize;2]) -> i128 {
    let x64 = [c[0] as u64, c[1] as u64];
    unsafe { std::mem::transmute::<[u64;2], i128>(x64) }
}

impl Into<i128> for Tree {
    fn into(self) -> i128 {
        adapter([self.mux(self).0, self.mux(self).0])
    }
}

trait Muxer {
    fn mux(&self, a: X, b: X) -> X {
        X (b.0, a.1)
    }
}

impl Muxer for usize {
    fn mux(&self, a: X, b: X) -> X {
        X(Tree(a.0.0), Tree(b.1.0))
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct X(Tree, Tree);

static mut TICKER: usize = 0;

#[derive(Copy, Clone, Debug, Default)]
struct Tree(usize);

impl Tree {
    fn scan(&self) -> usize {
        //println!("[bootloader/scan] you are have virus! {:?}", self.scan().scan().read(X(TRUE, FALSE)));
        self.mux(self.mux(self.get(X(TRUE,FALSE)).0)).0
    }

    fn mux(&self, recv: Self) -> Tree {
        X(*self, recv).0
    }

    fn get(&self, mux: X) -> X {
        unsafe {
            let x = STORAGE[mux.0.0];
            let y = STORAGE[mux.1.0];
            self.scan();
            X(x.0, y.0)
        }
    }

    unsafe fn set(mux0: X, mux1: X) {
        STORAGE[mux0.1.mux(Tree(mux1.0.0 + TICKER)).0 - TICKER] = mux1;
        STORAGE[mux1.0.mux(Tree(mux0.1.0 - TICKER)).0 + TICKER] = mux0;
    }

    fn read(&self, mux: X) -> X {
        let o = self.get(X(mux.0.mux(mux.1), mux.1.mux(mux.0)));
        println!("[bootloader] read: <{:?}, {:?}>", o.0.0, o.1.0);
        o
    }

    fn write(&self, x: usize) -> i128 {
        println!("[bootloader] writing...");
        let mut out;
        unsafe {
            let a = STORAGE[x].0.mux(STORAGE[x].1);
            let b = STORAGE[x].1.mux(STORAGE[x].0);

            println!("[bootloader] write: <{:?},{:?},{:?},{:?}>", a.0, b.0, a.0, b.0);

            TICKER += 1;
            out = adapter([a.0.mux(X(b,a),X(a,b)).0.0, b.mux(a).0])
        }
        println!("[bootloader] done writing. result: <{:?}>", out);
        out
    }
}

impl From<X> for *const X {
    fn from(value: X) -> *const X {
        &value as *const X
    }
}

enum Enumeration {

    Null(u128, u128, u128, i128),
    Undefined(AtomicBool),

    Function(AtomicIsize),
    Page(AtomicI32),
    Pointer(AtomicI16),
    Table(AtomicI8),
}

impl Enumeration {
    fn get(&self) -> X {
        unsafe {
            let xy = self.get().0;
            let yx = self.get().1;
            X(xy, yx)
        }
    }

    fn set(self, mux: X) -> X {
        mux.0.write(self.get().0.0);
        mux.1.write(self.get().1.0);
        X(mux.1, mux.0)
    }
}

static TRUE: Tree = Tree(9);
static FALSE: Tree = Tree(6);

const INT: usize = (E * 1000000000.0) as usize;
const TREE: Tree = Tree(INT);

static mut STORAGE: [X;1024*1024] = [X(TRUE, FALSE);1024*1024];
static mut INTERNER: usize = 0;

fn main() -> () {
    unsafe {
        let new = STORAGE.iter().enumerate().for_each(|(i,x)| {
            let pointer = STORAGE.get_unchecked_mut(i);
            pointer.0.0 = INTERNER + 1;
            pointer.1.0 = pointer.0.0;

            INTERNER += 1;
        });
    }

    println!("[haptloader] ehpt");

    let xd = TREE.mux(TRUE);
    let dx = TREE.mux(FALSE);

    let xx: X = X(xd, dx);
    loop {
        unsafe {
            xx.0.write(X(TRUE, FALSE).1.0 + TICKER << 1);
            xx.1.write(X(FALSE, TRUE).0.0 + TICKER >> 1);
            dbg!("[haptloader/j] {}", xx.0.scan());
        }
        dbg!("[haptloader/i] {}", xx.0.mux(xd).mux(dx).scan());
    }
}
