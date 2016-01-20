// lifetime elision rules
// Each elided lifetime in a function's arcuments becomes a distinct lifetime parameter.


fn print1(s: &str) {
    println!("{}", s);
}

fn print2<'a>(s: &'a str) {
    println!("{}", s);
}

fn main() {
    let s: &str;

    s = "hello";

    print1(s);
    print2(s);
}
