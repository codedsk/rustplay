// lifetime elision
// Each elided lifetime in a function's arguments becomes a distinct lifetime parameter

fn debug1(lvl: u32, s: &str) {
    println!("{} {}",lvl,s);
}

fn debug2<'a>(lvl: u32, s: &'a str) {
    println!("{} {}",lvl,s);
}

fn main() {
    let lvl: u32 = 14;
    let s: &str = "hello";

    debug1(lvl,s);
    debug2(lvl,s);
}
