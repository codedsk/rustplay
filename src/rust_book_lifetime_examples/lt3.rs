
fn substr1(s: &str, u: usize) -> &str {
    &s[0 .. u]
}

fn substr2<'a>(s: &'a str, u:usize) -> &'a str {
    &s[0 .. u]
}

fn main() {
    let s: &str = "hello";
    let u: usize = 3;
    let r1: &str;
    let r2: &str;

    r1 = substr1(s,u);
    r2 = substr2(s,u);

    println!("{}",r1);
    println!("{}",r2);
}
