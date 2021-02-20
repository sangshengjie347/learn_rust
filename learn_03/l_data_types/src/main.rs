/*
    Rust是一个静态类型的语言，在编译的时候就必须要知道各个值的类型
    编译器可以帮我们推断值的类型，但是一个值存在多种类型可能时，我们必须要添加类型注解

    u8: 无符号的整数 关联的值占据8比特位  0~255
    i8：有符号的整数 -128~127

    整型溢出  let a: u8 = 256;
    debug模式下编译不通过
    release模式下256 - 0  257 -1 258 -2

*/



fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess is {}", guess);

    let a: i32 = -1;  //显式类型注解
    let b = 10;  //默认i32类型

    let c = 12.22;  //默认f64
    let d = 12u8; //类型后缀 除了byte以外都可以使用类型后缀
    let e = 2.021; //默认f64
    let f: f32 = 2.021; //f32
    let g = 2.021f32; //f32
    //f32 是单精度浮点数，f64 是双精度浮点数。

    /*====================================*/
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    //布尔类型  true false

    let t = true;
    let f = false;

    //字符类型
    //在 Rust 中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值

    let e = '🐽';

    let e = '中';
    println!("E is {}", e);


    /*==============================*/

    //元组类型 元组长度固定

    let tup: (i32, f64, u8) = (500, 6.4, 10);

    //用模式匹配来解构元组值
    println!("0 is {} , 1 is {} , 2 is {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;

    println!("x is {} , y is {} , z is {} ", x, y, z);

    //数组  长度固定

    let a = [1, 2, 3, 4, 5];

    let a = [3; 5];  //索引就是0-4
    let a0 = a[4];
    println!("A0 is {}", a0);
}
