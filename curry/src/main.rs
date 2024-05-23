fn add(x: u32) -> impl Fn(u32) -> u32 {
    move |y| x + y
}


fn main() {
    let add2 = add(2);

    dbg!(add2(3));
    dbg!(add(2)(3));

    let power = |x: u32| move |y: u32| x.pow(y);

    println!("Или используя inline-замыкания:");
    dbg!(power(5)(3));
}
