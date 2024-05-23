use std::{ops::Add, cmp::PartialOrd, time::Duration};

/// Сигма-запись
fn sigma<T: Add<Output = T> + Default + PartialOrd>(left: T, right: T, term: fn(&T) -> T, next: fn(&T) -> T) -> T
{
    if left > right {
        return T::default();
    }

    term(&left) + sigma(next(&left), right, term, next)
}


fn accumulate<T: Default + PartialOrd>(left: T, right: T, term: fn(&T) -> T, next: fn(&T) -> T, combiner: fn(T, T) -> T) -> T 
{
    if next(&left) > right {
        term(&left)
    } else {
        combiner(term(&left), accumulate(next(&left), right, term, next, combiner))
    }
}

fn expt(base: i32, counter: i32, product: i32) -> i32 {
    if counter == 0 {
        product
    } else {
        expt(base, counter-1, base * product)
    }
}

/// Возведение в куб
fn cube(x: &i8) -> i8 {
    x.pow(3)
}


fn main() {
    let left_bound: i8 = 0;
    let right_bound: i8 = 5;

    let s1 = sigma(left_bound, right_bound, cube, |x: &i8| { *x+3 });

    println!("Сумма кубов от {left_bound} до {right_bound} с шагом 3 равна {s1}");

    // Также работает и с другими типами данных
    let dur1: Duration = Duration::new(5, 0);
    let dur2: Duration = Duration::new(10, 0);

    let s2 = sigma(
        dur1, 
        dur2, 
        |x| { *x * 60 }, // терм
        |x| { *x + Duration::new(1,0) } //следующее значение
    );

    println!("Сумма последовательности временных отрезков от {} минут до {} минут равна {} секунд", dur1.as_secs(), dur2.as_secs(), s2.as_secs());

    // обобщение сигма-записи
    let s3 = accumulate(
        1, 
        9, 
        |x| { *x },     // терм
        |x| { *x+2 },   // следующее значение
        |x,y| { x*y }   // функция комбинирования
    );

    println!("Произведение чисел от 1 до 9 с шагом 2 равно {s3}");

    // Линейное возведение в степень
    let s4 = expt(5, 4, 1);

    println!("5 в степени 4: {s4}");
}
