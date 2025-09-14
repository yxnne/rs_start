
mod front_of_house;
mod rect;
mod collection;
mod err;
mod traits;

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
