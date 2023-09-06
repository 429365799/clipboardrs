use std::time::Duration;

use api::{ClipboardData, ClipboardFile};

pub mod api;
pub mod listener;

use listener::ClipboardListen;

fn main() {
    let img =
        image::open("C:/Users/vs429/Pictures/v2-d55549366b52cde19c93835cfa2a58c9_r.jpg").unwrap();

    let files = vec![ClipboardFile {
        path: String::from("C:/Users/vs429/Downloads/21春本科工商管理(1)/21春本科工商管理/管理案例分析/形考任务3.docx"),
        size: 0,
    }];

    let data = ClipboardData {
        text: Some("qqqqq".to_string()),
        html: Some(
            "<div>1春本科工商管理(1)/21春本科工商管理/管理案例分析/形考任务3</div>".to_string(),
        ),
        image: Some(img),
        files: Some(files),
    };

    api::write_clipboard_data(data, true).unwrap();

    println!("开始");
    let handle = ClipboardListen::run(move || {
        println!("剪贴板更新！");
    });

    // std::thread::sleep(Duration::from_secs(10));
    handle.join().unwrap();

    println!("结束");
}
