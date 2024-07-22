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
