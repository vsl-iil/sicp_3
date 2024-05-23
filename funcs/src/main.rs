fn sum<T>(x: T, y: T) -> T {
    return x + y;
}

fn pow3(x: i32) -> i32 {
    x.pow(3)
}

fn ln(x: i32) -> i32 {
    (x as f32).ln() as i32
}

fn func3(f1: fn(i32) -> i32, f2: fn(i32) -> i32, x: i32, y: i32) -> i32 {
    f1(x) + f2(y)
}


fn main() {
    println!("func3(pow3, ln, 3, 4) = {res}", res = func3(pow3, ln, 3, 4));
}
