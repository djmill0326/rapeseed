/* #!/bin/curl http://ehpt:666/haptloader/v1 >> /bin/fish */

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr, AtomicUsize};

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

static mut MAP: VecDeque<X> = VecDeque::new();

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

type E = Enumeration;

fn exe(depth: usize) -> X {
    // init a pointers;
    let a = Tree(00).mux(Tree(depth));
    let b = Tree(01).mux(a.0);
    let c = Tree(02).mux(b.0);
    let d = Tree(03).mux(c.0);
    let e = Tree(04).mux(d.0);
    let f = Tree(05).mux(e.0);
    let g = Tree(06).mux(f.0);
    let h = Tree(07).mux(g.0);
    let i = Tree(08).mux(h.0);
    let j = Tree(09).mux(i.0);
    let k = Tree(10).mux(j.0);
    let l = Tree(11).mux(k.0);
    let m = Tree(12).mux(l.0);
    let n = Tree(13).mux(m.0);
    let o = Tree(14).mux(n.0);
    let p = Tree(15).mux(o.0);
    let q = Tree(16).mux(p.0);
    let r = Tree(17).mux(q.0);
    let s = Tree(18).mux(r.0);
    let t = Tree(19).mux(s.0);
    let u = Tree(20).mux(t.0);
    let v = Tree(21).mux(u.0);
    let w = Tree(22).mux(v.0);
    let x = Tree(23).mux(w.0);
    let y = Tree(24).mux(x.0);
    let z = Tree(25).mux(y.0);
    let az = a.0.mux(z.0);
    let za = z.0.mux(a.0);
    let snake_case = X(za.0, za.1);
    let camel_case = X(az.0, az.1);
    let tri_case = X(snake_case.0, camel_case.1);
    let quad = X(camel_case.0, snake_case.1);
    let pen = X(quad.0, tri_case.1);
    let test = X(tri_case.1, quad.0);
    let paperclip = (pen, test);
    let lynchpin = [
        a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r,s,t,u,v,w,x,y,z,y,x,w,v,u,t,s,r,q,p,o,n,m,l,k,j,i,h,g,f,e,d,c,b,a
    ];

    println!("[haptloader/whar] whar.");

    lynchpin.iter().for_each(|x| {
        lynchpin.iter().rev().for_each(|y| {
            let a = paperclip.0.0.mux(x.0);
            let b = paperclip.0.1.mux(y.0);
            let c = paperclip.1.0.mux(x.1);
            let d = paperclip.1.1.mux(y.1);
            let e = paperclip.0.0.mux(x.1);
            let f = paperclip.0.1.mux(y.1);
            let g = paperclip.1.0.mux(x.0);
            let h = paperclip.1.1.mux(y.0);
            let clip = [h, g, f, e, d, c, b, a];
            let paper = [a, b, c, d, e, f, g, h];
            let paper_clip = [paper, clip];
            paper_clip.map(|i| {
                pen.0.write(x.0.0 + y.1.0);
                test.1.write(y.0.0 + x.1.0);
                test.0.write(x.0.0 + y.1.0);
                pen.0.write(y.0.0 + x.1.0);
            });
        });
    });
    test
}

fn main() -> () {
    println!("[haptloader] ehpt");
    exe(666);
}
