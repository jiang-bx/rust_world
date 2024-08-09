#![allow(unused_variables)]
#![allow(dead_code)]

#[test]
fn test1() {
    let m: (i32, f64, u8) = (500, 5.4, 222);

    dbg!(m);
}

#[test]
fn test2() {
    let m = (500, 5.4, 222);
    let (x, y, z) = m;
    dbg!(x, y, z);
}

#[test]
fn test3() {
    let m = (500, 5.4, 222);

    dbg!(m.0, m.1, m.2);
}

#[test]
fn test4() {
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
    }

    let user = User {
        active: true,
        username: String::from("fsadf"),
    };

    // 所有权被拿走
    dbg!(&user);

    println!("{}", &user.username);
}

#[test]
fn test5() {
    #[derive(Debug)]
    struct User {
        dd: i32,
        active: bool,
        username: String,
    }

    fn build_user(dd: i32, active: bool) -> User {
        User {
            dd,
            active,
            username: String::from("dsfafa"),
        }
    }

    let u1 = build_user(1, false);
    let u2 = User {
        active: true,
        username: String::from("222222"),
        // 其他未声明的值全部由 u1 进行初始化
        ..u1
    };

    dbg!(u1);
    dbg!(u2);
}

#[test]
fn test6() {
    #[derive(Debug)]
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
    }

    // 访问 枚举值
    let h = PokerSuit::Hearts;

    dbg!(h);
}

#[test]
fn test7() {
    #[derive(Debug)]
    enum PokerCard {
        Clubs(u8),
        Spades(u8),
        Diamonds(u8),
        Hearts(u8),
    }

    // 访问 枚举值
    let h = PokerCard::Hearts(3);

    dbg!(h);

    let a = Some("fsdf");
    let b = Some(5);

    dbg!(a, b);
}

#[test]
fn test8() {
    let a = [1, 22, 3];
    dbg!(a);

    let b: [i32; 2] = [2, 3];
    dbg!(b);

    let c = ["fff"; 5];
    dbg!(c);

    let d = &c[..2];
    let e = &b[..1];

    dbg!(d, e);
}
