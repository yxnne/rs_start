mod front_of_house;
mod rect;
mod collection;
mod err;
mod traits;
mod smart_pointer;
mod muti_thread;
mod match_mod;

use std::fmt::Display;

use crate::traits::summary::Summary;  // 编译器说“method not found”，并不是 summarize 没写，而是 它不知道 SocialPost 实现了 Summary —— 也就是 trait 的作用域没引进来。
use crate::traits::summary::notify;  
use crate::front_of_house::hosting;
use rect::Rectangle;
use collection::vec;
use collection::str;
use collection::map;
use err::panic_learn;
use err::res;
use traits::post;
use smart_pointer::learn_box;
use smart_pointer::learn_box::List::{Cons, Nil};
use muti_thread::learn_thread;
use muti_thread::learn_co;
use match_mod::learn_match;



static mut COUNTER: u32 = 0;

/// SAFETY: 同时在多个线程调用这个方法是未定义的行为，所以你*必须*保证同一时间只
/// 有一个线程在调用它。
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Hello, world!");
    hosting::add_to_waitlist();

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // 变量私有

    let rect1 = Rectangle::new(30, 50);

    let rect2 = Rectangle::square(100);
    

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // 此函数调用直接panic了
    // panic_learn::try_panic();

    // panic_learn::panic_buffer_overread();


    vec::try_vec();

    str::try_str();

    str::link_str_lost();

    str::link_str_safe();

    str::try_str_loop();

    map::try_map();

    map::try_map_item_statistic();

    // res::simple_expect_read_test_txt();


    println!("trait try start -----");
    let post = post::SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());

    notify(&post);

    let str_l1 = String::from("I'm learning rust");
    let str_l2 = "I'm learning rust long long";
    let ann = String::from("try lifetime");

    let longest = longest_with_an_announcement(
        &str_l1, 
        &str_l2,
        ann
    );

    println!("The longest string is: {}", longest);


    // 闭包
    let mut rects = [
        Rectangle::new(100, 50),
        Rectangle::new(2, 500),
        Rectangle::new(10, 500),
    ];
    
    rects.sort_by_key(|r| r.area());

    println!("rects {:?}", rects);


    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);


    println!("learn box start -----");

    learn_box::try_box();

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list: {:?}", list);

    println!("learn thread start -----");

    learn_thread::try_thread();

    learn_thread::try_move();

    learn_thread::try_mutex();

    // learn_co::try_co();

    println!("learn match start ----- 0000");



    learn_match::try_match1();

    learn_match::try_match2();

    
    

    unsafe {
        // SAFETY: 它只在 `main` 这一个线程被调用。
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}


// 泛型生命周期
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
