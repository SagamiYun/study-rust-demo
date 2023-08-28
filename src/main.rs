use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::Duration;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use async_std::task;
use async_std::task::spawn;
use crate::circular_reference::create_owner_gadget;
use crate::error::{a_depth_error, a_easy_error};
use crate::life_time::test_lifetime;
use crate::r#unsafe::array_to_slice;
use crate::tcp_stream::handle_connection;
use crate::thread::thread_main;
use crate::tree::Node;
use crate::wrapper::Wrapper;
use crate::vector::{IpAddr, V4};
use crate::vector::V6;

mod wrapper;
mod vector;
mod life_time;
mod circular_reference;
mod tree;
mod thread;
mod error;
mod r#unsafe;
mod tcp_stream;
// mod web_server;
// mod cacher;

fn complications() {
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

        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}",
                      record, fields);
        }

        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

fn variables() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn citation() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn external_types_impl_exterior_features() {
    let mut w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    w.push(String::from("new"));
    println!("w = {}", w);
}

fn store_different_types_of_vector() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

fn circular_reference_tree () {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

#[async_std::main]
async fn main() {
    // 复杂例子
    // complications();

    // 可变“常量”
    // variables();

    // 引用与解引用（指针与地址）
    // citation()

    // 在外部类型上实现外部特征(new type)
    // external_types_impl_exterior_features()

    // 实现将不同对象加入到vector中(特征对象)
    // store_different_types_of_vector()

    // 进阶lifetime
    // test_lifetime()

    // 循环引用
    // create_owner_gadget()
    // 使用weak完成tree数据结构
    // circular_reference_tree()

    // 线程
    // thread_main()

    // 最简单的错误
    // a_easy_error()

    // 更详尽的错误
    // a_depth_error()

    // unsafe的获取两个数组的切片
    // array_to_slice()

    // async web server
    // loop_listen()
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}
