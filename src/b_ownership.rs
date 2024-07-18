#[test]
fn a() {
    // 可变的引用字符串对象
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}", s);
}

#[test]
fn b() {
    let s1 = String::from("aaa");
    let s2 = s1;

    // println!("{}", s1);
    println!("{}", s2);
}

#[test]
fn c() {
    // 不可变引用, 实际上就是 字符串的指针, x 持有的是固定字符串的指针, 并且无法修改
    let x = "hell";
    let y = x;

    println!("x = {}, y = {}", x, y);
}

#[test]
fn d() {
    let s1 = String::from("aaa");
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);
}

#[test]
fn e() {
    // 浅拷贝
    let s1 = 1;
    let s2 = s1; // 将 s1 的值复制了一份, 并且和 s2 绑定, 所有权是 s2

    println!("{}", s1);
    println!("{}", s2);
}

#[test]
fn f() {
    fn make_copy(x: i32) {
        println!("{}", x)
    } // 函数结束, 将 x 移除作用域

    fn take_some(str: String) {
        println!("{}", str)
    } // 函数结束, 调用 drop,  str 所占用的内存被释放

    let s = String::from("hello");

    take_some(s); // s 的值转移到了函数中

    // 报错
    // println!("在move进函数后继续使用s: {}", s);

    let x = 5;
    make_copy(x);
}

#[test]
fn g() {
    let x = 5;
    let y = &x;
    let z = *y;

    assert_eq!(5, x);
    assert_eq!(5, *&x);
    // 报错
    // assert_eq!(5, y);
    assert_eq!(5, *y);

    assert_eq!(5, z);
}
