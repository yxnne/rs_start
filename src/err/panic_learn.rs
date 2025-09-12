pub fn try_panic() {
    panic!("crash and burn");
}

// RUST_BACKTRACE=1 cargo run

pub fn panic_buffer_overread() {
    let v = vec![1, 2, 3];
    v[99];
}