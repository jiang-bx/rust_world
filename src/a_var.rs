fn _a() {
    let mut num = 3;
    println!("{}", num);
    num = 5;
    println!("{}", num);
}

fn _b() {
    let (x, mut y): (bool, bool) = (true, false);

    println!("{:?}, {:?}", x, y);

    y = true;

    assert_eq!(x, y)
}

struct Struct {
    e: i32,
}

#[test]
fn c() {
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

#[test]
fn d() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("fd{}", x)
    }

    println!("dd{}", x);

    // let mut s = "  ";
    // s = s.len();

    assert_eq!(100u8.saturating_add(1), 101);

    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

    let z = 3 / 2;
    println!("{}", z);

    let q = 1000_999;
    println!("{}", q);

    for i in 1..=5 {
        println!("{}", i);
    }

    let s: char = '中';
    println!("{}", s);
}

#[test]
fn e() {
    let x = 1;
    let y = {
        let x = 2;
        x * 2
    };

    // 三元表达式
    let z = if x * y % 2 == 1 { "odd" } else { "even" };

    println!("**** {} *****", z);
}

#[test]
fn f() {
    fn add_f(i: i32, j: i32) -> i32 {
        i + j
    }

    println!("{}", add_f(1, 2));

    fn plus_or_minus(x: i32) -> i32 {
        if x > 5 {
            return x - 5;
        }

        x + 5
    }

    println!("{}", plus_or_minus(11));
}

pub fn _test() {
    _b()
}

#[test]
fn test() {
    _b()
}
