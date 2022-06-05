pub fn fizz_buzz1() {
    let mut x = 1;

    while x <= 100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
        x += 1;
    }
}

pub fn fizz_buzz2() {
    for x in 1 .. 101 {     // Repeat from 1 to 100
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}

pub fn fizz_buzz3() {
    for x in 1 ..= 100 {    // Repeat from 1 to 100
        match x % 15 {      // Pattern match
            0 => println!("FizzBuzz"),
            3 | 6 | 9 | 12 => println!("Fizz"),
            5 | 10 => println!("Buzz"),
            _ => println!("{}", x),     // Wildcard pattern
        }
    }
}

pub fn fizz_buzz4() {
    for x in 1 ..= 100 {
        match x {
            e if e % 15 == 0 => println!("FizzBuzz"),
            e if e % 3 == 0 => println!("Fizz"),
            e if e % 5 == 0 => println!("Buzz"),
            e => println!("{}", e),
        }
    }
}

pub fn fizz_buzz5() {
    for x in 1 ..= 100 {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _      => println!("{}", x),
        }
    }
}

pub fn fizz_buzz6() {
    for x in 1 ..= 100 {
        let s = match (x % 3, x % 5) {          // match is an expression.
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _      => x.to_string(),
        };
        println!("{}", s)
    }
}

pub fn fizz_buzz7() {
    for x in 1 ..= 100 {
        let tmp;
        let s = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz",
            (0, _) => "Fizz",
            (_, 0) => "Buzz",
            _      => {tmp = x.to_string(); &tmp},      // Bind `tmp` to `x.to_string()` and take a reference to it.
        };
        println!("{}", s)
    }
}

pub fn fizz_buzz8() {
    let fz = |x: i32| {     // Difining Closure
        match (x % 3, x % 5) {
            (0, 0) => format!("FizzBuzz"),
            (0, _) => format!("Fizz"),
            (_, 0) => format!("Buzz"),
            _      => x.to_string(),
        }
    };
    (1 ..= 100).map(fz).for_each(|x| println!("{}", x))     // Using higher-order functions and the closure.
}

pub fn fizz_buzz9() {
    let res = (1 ..= 100)
        .fold(format!(""), |buf, x| {       // Perform string convolution.
            match (x % 3, x % 5) {
                (0, 0) => format!("{}FizzBuzz\n", buf),
                (0, _) => format!("{}Fizz\n", buf),
                (_, 0) => format!("{}Buzz\n", buf),
                _      => format!("{}{}\n", buf, x),
            }
        });
    println!("{}", res)
}

pub fn fizz_buzz10() {
    fn fz(x: i32) -> String {
        match (x % 3, x % 5) {
            (0, 0) => format!("FizzBuzz"),
            (0, _) => format!("Fizz"),
            (_, 0) => format!("Buzz"),
            _      => x.to_string(),
        }
    }
    let res = (1 ..= 100).map(fz).collect::<Vec<_>>().join("\n");
    println!("{}", res)
}

pub fn fizz_buzz11() {
    use std::ops::Rem;

    fn fz<T>(x: T, div_a: T, div_b: T, zero: T) -> String
    where T: Rem<T, Output=T> + Eq + Copy + ToString {
        match (x % div_a == zero, x % div_b == zero) {
            (true, true) => format!("FizzBuzz"),
            (true, _   ) => format!("Fizz"),
            (_   , true) => format!("Buzz"),
            _            => x.to_string(),
        }
    }
    (1 ..= 100).map(|x: u32| fz(x, 3, 5, 0)).for_each(|x| println!("{}", x));
}
