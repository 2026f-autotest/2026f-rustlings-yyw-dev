// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; //两个可变引用的生命周期没有重叠，所以是可以的
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
