// 编译器属性, 忽略未使用的变量
#![allow(unused_variables)]
#![allow(dead_code)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

#[test]
fn test1() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);

    // read(&mut f1, &mut vec![]);

    close(&mut f1);
}

#[test]
fn test2() {
    let s = String::from("heeee eee");
    let h = &s[0..5];
    let w = &s[5..];

    println!("{}", h);
    println!("{}", w)
}

#[test]
fn test3() {
    let s = String::from("中国人");
    let h = &s[..3];

    println!("slice 取 index 为3, {}", h);

    for c in s.chars() {
        println!("字符: {}", c)
    }

    for b in s.bytes() {
        // 一个中文字符占 三个字节, 所以一共有 9 个字节
        println!("字节: {}", b)
    }
}

fn frist_world(s: &String) -> &str {
    return &s[..1];
}

#[test]
fn test4() {
    let mut s = String::from("asdfasfs");

    // 不可变引用
    let w = frist_world(&s);

    // 可变引用
    s.clear();

    // println!("{}", w);
}

#[test]
fn test5() {
    let mut s = String::from("中, 件, 非");

    s.push_str("fsfa");

    dbg!(s);
}

#[test]
fn test6() {
    let mut s = String::from("中, 件, 非");

    // 这里的 idx 要特别注意索引的问题, utf-8 编码边界不确定哦
    // ! 很容易越界访问报错
    s.insert_str(3, "fsfa");

    dbg!(s);
}

#[test]
fn test7() {
    let s = String::from("中, 件, 非");

    let n_s = s.replace("非", "进");

    dbg!(s);
    dbg!(n_s);
}

#[test]
fn test8() {
    let mut s = String::from("中, 件, 非");

    // 涉及到索引的要注意
    s.replace_range(..3, "进");
    dbg!(s);
}

#[test]
fn test9() {
    let s = String::from("中, 件, 非");
    let s2 = "21313";
    let s3 = format!("{} + {}", s, s2);

    dbg!(s3);
}

#[test]
fn test10() {
    let s = String::from("中, 件, 非");
    let s2 = r#"#213'
    q \r   werw13"#;
    let s3 = format!("{} + {}", s, s2);

    for c in s3.chars() {
        println!("{}", c);
    }

    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    dbg!(s1);
}
