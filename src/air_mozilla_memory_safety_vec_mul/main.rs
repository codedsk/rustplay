// https://air.mozilla.org/guaranteeing-memory-safety-in-rust

fn vm(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut sum :i32 = 0;

    for (v1p, v2p) in v1.into_iter().zip(v2.into_iter()) {
        sum += (*v1p) * (*v2p);
    }

    sum
}

fn main() {
    let v1 = vec!(1,2,3,4,5);
    let v2 = vec!(1,2,3,4,5);

    let sum :i32 = vm(&v1, &v2);

    println!("v1 = {:?}\nv2 = {:?}\nv1*v2 = {}",v1,v2,sum);
}
