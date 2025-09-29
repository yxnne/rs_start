use async_std::task;          // 执行器
use std::time::Duration;

/// 模拟异步 HTTP：延迟 1 秒返回
async fn fake_get(url: &str) -> String {
    println!("start  fetching {}", url);
    task::sleep(Duration::from_secs(1)).await;
    println!("finish fetching {}", url);
    format!("body of {}", url)
}

pub fn try_co() {
    // 手动 block_on：把异步世界拉进同步 main
    let body = task::block_on(fake_get("https://example.com"));
    println!("got got got async: {}", body);
}