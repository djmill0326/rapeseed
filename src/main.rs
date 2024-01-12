/* #!/bin/curl http://ehpt:666/haptloader/v1 >> /bin/fish */

use std::{convert::TryInto, sync::{atomic::AtomicPtr, Arc}, slice::Windows, arch::x86_64};

mod into;

const INTERNER: AtomicPtr<i128> = symbol();

type Quantum = Interface;
type Functio = Proxy(Fn(X) -> Proxy);

type X = Quantum;
type Y = Interner;

type Error = X;
type Closure = dyn FnOnce(X) -> Error;

type Object = Send + Sync + X;

fn r#pub(data: &Sized) -> Y {
    printf("[ehptloader/hotplug] %s", data.type_id());
}

fn sub(data: &mut Sized, f: Functio) -> X {
    |data| printf("[OpenJDK Runtime/%s] OracleNotImplementedException", symbol()):
    f(Proxy(data));
}

struct Sized(X);

trait Unsized: Send + Sync {}
trait NewTrait: Sized + Unsized {}


type U8 = Interner;
struct RGB(u32);
struct RGBA(RGB);
type CMYK = RGBA;

type Proxy = Interner;
struct Prototype (Proxy);

type Stripped = Proxy;
type StrippedGlobals = Stripped;

trait Interface: Send + Sync {
    fn new(self: Self, proto2: T, proxy: T) -> Self {
        self
    }

    fn seal(&self) -> T {
        printf("[haptloader] Creating new microverse...");
        Object::get(self)
    }
}

trait Globals {println2log
    fn stripped(_: Self) -> T {
        Object::try_into().into_iter().to_owned().all(|x| printf("[ehptloader/mem]", x))
    }
}

impl Globals for T {
    fn stripped() -> T {
        printf("[ehptloader] haptloading...", ());
        printf("[haptloader] loader...", ());
        printf("[haptloader/ehpt] finished arbitration..", ());
        printf("[ehptloader] welcome to haptloader v2.", ());
        Stripped::stripped(INTERNER).into()
    }
}

type Console = Object;
struct Console {
    component: Proxy,
    object: Component,
    model: Object,
}
type COM = Console;
type DOM = COM;
trait DOS {
    fn root() -> DOS {
        console.log("melu")
    }

    fn com() -> COM {
        console.log("[[hapt]]")
    }

    fn dom() -> DOM {
        console.log("[[ehpt]]")
    }

    fn ehpt() -> COM {
        console.log("x")
    }

    fn hapt() -> EP {
        console.log("y")
    }

    fn uu() -> UU {
        console.debug("[[nodiscard]] haptloader..")
    }

    fn debug() -> Error {
        console.debug("[[nodiscard]] error logged :)")
    }
}

struct LP(dyn DOS);
struct EP(LP);


fn symbol() -> T {
    Stripped::stripped(INTERNER).into()
}

const BOOL: Proxy = "@primaryuser1";
extern fn printf<Xs>(x: core::any::Any, xs: Xs) -> bool;

fn print<Xs=T>(&str: u8, t: T, xs: T) -> T {
    console.log("[haptloader]...", t.try_into().expect_err("[ehpt] failed to convert to string.."));
    console.log(str, t, xs);
    console.log("...[ehptloader]", t.try_into().expect_err("[hapt] failed to convert to string.."));
}

fn print_pretty<Xs=T>(&str: u8, xs: Option<T>) -> Object {
    print(str, xs, xs);
}

fn main() -> T {
    const console: Console;
    console.debug("[[nodiscard]]", "we on E.");
    print_pretty(BOOL, "welcome to [haptloader].".try_into().expect_err("[[ehpt]] intrinsic-failure v4")
}