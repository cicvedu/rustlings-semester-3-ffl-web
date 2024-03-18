// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]

fn main() {
    let my_option: Option<()> = None;
    match my_option {
        Some(_) => {
            println!("Option contains a value!");
        }
        None => {
            println!("Option is empty!");
        }
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    my_empty_vec.resize(5, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    let mut a=0;
    // Let's swap these two!
a=value_a;
value_a=value_b;
value_b=a;
    println!("value a: {}; value b: {}", value_a, value_b);
}
/*
这段代码出错的原因是尝试在一个None值上调用Option::unwrap()方法，导致程序发生了panic。unwrap()方法会从Option类型中提取Some的值，如果是None，则会导致panic。

对于代码中的let my_option: Option<()> = None;部分，这段代码的意思是创建一个类型为Option<()>的变量my_option并将其赋值为None，表示这个Option类型的变量是空的，不包含任何实际的值。

*/