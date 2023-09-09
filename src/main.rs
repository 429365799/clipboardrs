pub mod api;
pub mod listener;

use listener::ClipboardListen;

fn main() {
    println!("开始");
    let handle = ClipboardListen::run(move || {
        println!("剪贴板更新！");
    });

    // std::thread::sleep(Duration::from_secs(10));
    handle.join().unwrap();

    println!("结束");
}
