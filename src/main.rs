/* #!/bin/curl http://ehpt:666/haptloader/v1 >> /bin/fish */

trait Scanner {
    fn scan(self) -> Tree;
}

impl Scanner for Tree {
    fn scan(self) -> Tree {
        println!("[haptloader] innit");
        self
    }
}

impl Into<usize> for Tree {
    fn into(self) -> usize {
        self.0
    }
}

trait Muxer {
    fn mux(a: X, b: X) -> X;
}

impl Muxer for X {
    fn mux(a: X, b: X) -> X {
        X (a.1, b.0)
    }  
}

#[derive(Copy, Clone, Debug, Default)]
struct X(Tree, Tree);

#[derive(Copy, Clone, Debug, Default)]
struct Tree(usize);

impl Tree {
    fn scan(&self) -> Self {
        println!("[haptloader/scan] you are have virus! {:?}", self.0);
        self.clone()
    }

    fn mux(&self, recv: Self) -> X {
        X(self.scan(), recv.scan())
    }

    fn get(&self, mux: X) -> X {
        unsafe {
            let x = MAP[mux.0.0];
            let y = MAP[mux.1.0];
            self.scan();
            X(x.0, y.0)
        }
    }

    unsafe fn set(mux0: X, mux1: X) {
        MAP[mux0.1.mux(mux1.0).1.0] = mux1;
        MAP[mux1.0.mux(mux0.1).0.0] = mux0;
    }

    fn read(&self, mux: X) -> X {
        let o = self.get(X(mux.0.mux(mux.1).0, mux.1.mux(mux.0).1));
        println!("[ehptloader] read: <{:?}, {:?}>", o.0.0, o.1.0);
        o
    }

    fn write(&self, x: usize) {
        unsafe {
            let a = MAP[x].0.mux(MAP[x].1);
            let b = MAP[x].1.mux(MAP[x].0);
            println!("[haptloader] write: <{:?},{:?},{:?},{:?}>", a.0.0, b.0.0, a.1.0, b.1.0);
        }
    }
}

impl From<X> for *const X {
    fn from(value: X) -> *const X {
        &value as *const X
    }
}

static mut MAP: Vec<X> = Vec::new();

fn main() -> () {
    let zero = Tree(0);
    let one = Tree(1);
    //zero.write(one.0);
    one.read(zero.mux(one));
    println!("[haptloader] ehpt");
}