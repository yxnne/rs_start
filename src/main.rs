mod front_of_house;
mod rect;
mod collection;
mod err;

use crate::front_of_house::hosting;
use rect::Rectangle;
use collection::vec;
use collection::str;
use collection::map;
use err::panic_learn;
use err::res;



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

    res::simple_expect_read_test_txt();
}
