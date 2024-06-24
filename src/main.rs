fn main() {
    println!("Hello, world!");
    
    let ref i = String::from("value"); // 创建一个可变引用
    let j = i;
    let x = String::from("122");
    let y = &x;
    let mut z = j;
    println!("{:?} {:?}  {:?}", i,j,x); // 访问引用的值
}