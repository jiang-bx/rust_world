mod a_var;
mod b_ownership;
mod c_string;

mod d_tuple;

mod e_control;

fn _greet_world() {
    let sou = "Grr Goot";
    let ch = "世界, 你好";
    let en = "hello world";

    let reg = [sou, ch, en];

    for re in reg.iter() {
        println!("{}", &re);
    }
}

fn _test2() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        // 条件编译
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];

        // if let 是一个匹配表达式
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

fn main() {}
