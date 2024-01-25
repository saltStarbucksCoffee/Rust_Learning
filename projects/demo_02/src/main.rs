fn main() {
    /*
     * 常量
     */
    // const PI: f32 = 3.14;
    // println!("PI_VALUE:{}",PI);

    /*
     * 不可变变量的示范
     */
    // let a = "Hello,world!";
    // println!("a:{}",a);
    // a = "Hello,Rust";
    // println!("a:{}",a);

    /*
     * 可变变量的示范
     */
    // let mut a = "Hello,world!";
    // println!("a:{}",a);
    // a = "Hello,Rust";
    // println!("a:{}",a);

    /*
     * 使用下划线开头忽略未使用的变量
     */
    // let _b = "Hello,world!";
    // let c = "Hello,Rust";

    /*
     * 变量的解构
     */
    // let (a, mut b):(bool,bool) = (true,false);
    // // a = true 不可变， b=false 可变
    // println!("a:{},b:{}",a,b);
    // b = true;
    // assert_eq!(a,b);

    /*
     * 变量的覆盖(shadowing)
     */
    let sd = 1;
    let sd = sd+1;
    {
        let sd = sd * 5;
        println!("sd one:{}",sd);
    }
    println!("sd two:{}",sd);
}
