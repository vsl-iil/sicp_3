// Вариант 3
fn elem(x: f32) -> f32 {
    -(1.0 / x)
}

fn next(x: f32) -> f32 
{
    x+2.0
}

fn combine<T>(elem: T, next: T)
-> impl Fn(f32) -> f32
where T: Fn(f32) -> f32 + Clone
{
    move |x| elem(x) + elem(next(x))
} 

fn wrapper<T,F>(start: f32, end: f32, elem: T, next: T, comb: F) -> impl Fn(f32) -> f32 
where T: Fn(f32)->f32 + Clone,
      F: Fn(T, T) -> T
{
    if start < end {
        wrapper(start+1.0, end, comb(elem, next.clone()), next, comb)
    } else {
        comb(elem, next)
    }
}

fn main() {
    let f = wrapper(1.0, 2.0, elem, next, combine);
    dbg!(f(2.0));
}
