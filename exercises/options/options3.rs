// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


//struct结构体没有显示规定copy和clone，默认使用move进行所有权的更换
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    //模式匹配变量绑定值的时候，也会发生所有权的转移
    match y {
        //在这里加上ref表示对于所有权的不可变借用
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}

//rust里面的每一个等号都要考虑所有权的转移