/*
一.变量可变与不可变性
1. 变量默认是不可以改变的
2. 如果要改变变量为可变变量 需在变量名前加mut关键字

二.变量和常量的区别
1.定义不同 变量定义关键字是let 常量定义关键字是const
2.常量的名称默认全部大写，多个单词用下划线分隔
3.常量定义的时候你必须要显示的注明类型
4.常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。

三.隐藏(shadowing)
1. 对变量重新赋值时，没有使用let，就会产生编译错误
2. let对变量进行重新赋值或者计算后变量仍然是不可变的
3. let其实它是创建了一个新的变量，它可以是不同的数据类型，但是mut必须是同意数据类型
*/
fn main() {
    let mut x = 5;  //默认的i32
    let y = 2.0;  //默认的f64
    println!("x is {}", x);
    x = 6;
    x = x + 1;
    println!("x is {}", x);
    const MAX_POINT: i32 = 100_000;  //字面值可以通过插入下划线来提升可读性，不会影响值
    println!("MAX_POINT is {}", MAX_POINT);
    //const SUM: i32 = two_sum(2020,1);
    let sum = two_sum(2020, 1);
    println!("sum is {}", sum);

    /*==========================================*/
    let s = 6; //默认不可变变量
    println!("s is {}", s);
    let s = 7;
    println!("s is {}", s);
    let s = s + 1;
    println!("s is {}", s);

    /*==========================================*/
    let spaces = "    ";
    println!("spaces is {}", spaces);

    let spaces = spaces.len();
    println!("spaces is {}", spaces);

    let mut spaces = "    ";
    println!("spaces is {}", spaces);
    spaces = "AAAAAA";
    println!("spaces is {}", spaces);
    //spaces=spaces.len(); mut修改变量值必须为同一个类型
    let a = spaces.len();
    println!("a is {}", a);
}

fn two_sum(i: i32, j: i32) -> i32 {
    i + j
}