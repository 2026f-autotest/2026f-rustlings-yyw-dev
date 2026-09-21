// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());//建立一个新的String等价于to_string()
    string("nice weather".into());//看上下文决定都可以变
    string(format!("Interpolation {}", "Station"));//格式化宏返回String
    string_slice(&String::from("abc")[0..1]);//String变为&str切片
    string_slice("  hello there ".trim());//&str调用返回&str去除首尾空格
    string("Happy Monday!".to_string().replace("Mon", "Tues"));//String调用replace返回String
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); //&str调用，把所有字符变成小写，返回新的有所有权的String
}
