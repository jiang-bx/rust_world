#![allow(unused_variables)]
#![allow(dead_code)]

#[test]
fn test1() {
    let condition = true;

    // 类似 三运运算符
    let num = if condition { 5 } else { 6 };

    dbg!(num);

    let n = 6;

    if n % 4 == 0 {
        println!("1 -- {}", n % 4);
    } else if n % 3 == 0 {
        println!("2 -- {}", n % 3);
    } else {
        println!("3 -- {}", n % 2);
    }
}

#[test]
fn test2() {
    for i in 0..5 {
        println!("i = {}", i);
    }

    let a = [1, 2, 3, 4, 5];

    for (i, v) in a.iter().enumerate() {
        println!("第 {} 个 = {}", i, v);
    }

    // 转移所有权
    for item in a {
        println!("{}", item);
    }

    // 不可变引用
    for item in &a {
        println!("{}", item);
    }
}

#[test]
fn test3() {
    let s = String::from("fsadfa. 中文");
    for ch in s.chars() {
        println!("{}", ch)
    }

    let mut b = [3; 5];
    b[0] = 1;
    dbg!(b);

    let c = &b[2..];
    dbg!(c);

    for item in c {
        println!("{}", item);
    }
}

#[test]
fn test4() {
    let a = [1, 2, 3, 4, 5];
    let mut i = 0;

    while i < a.len() {
        if i == 2 {
            i += 1;
            continue;
        }
        println!("o = {}", a[i]);
        i += 1;
    }

    let mut n = 0;

    loop {
        println!("持续中...");
        if n >= 5 {
            println!("结束");
            break;
        }
        n += 1;
    }
}

#[test]
fn test5() {
    enum Direction {
        East,
        West,
        North,
        South,
    }

    let dire = Direction::West;

    match dire {
        Direction::East => println!("East"),

        Direction::North | Direction::West => {
            println!("N or W")
        }

        _ => {
            println!("else")
        }
    };
}

#[test]
fn test6() {
    #[derive(Debug)]
    enum UsState {
        Alibama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Quarter(state) => {
                println!("State quarter form {:?}!", state);
                25
            }
            Coin::Dime => 10,
            Coin::Nickel => 5,
            _ => 1,
        }
    }

    let a = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", a);

    let b = value_in_cents(Coin::Nickel);
    println!("{}", b);
}

#[test]
fn test7() {
    #[derive(Debug)]
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let actions = [
        Action::Say("heee".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(0, 0, 0),
    ];

    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("x = {}, y = {}", x, y);
            }
            Action::ChangeColorRGB(r, g, b) => {
                println!("r = {}, g = {}, b = {}", r, g, b);
            }
        }
    }
}

#[test]
fn test8() {
    let a = Some(3_u8);
    match a {
        Some(3) => println!("therr"),
        _ => (),
    }

    let b = Some(4_u16);
    if let Some(4) = b {
        println!("fore");
    }
}

#[test]
fn test9() {
    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }

    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    let d = v.iter().filter(|x| matches!(x, MyEnum::Foo));

    for item in d {
        dbg!(item);
    }
}
