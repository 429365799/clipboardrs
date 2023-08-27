pub mod api;

fn main() {
    let img = image::open("D:/workspace/ws_rust/clipboardrs/out.bmp").unwrap();
    api::write_image(img).unwrap();

    // api::write_text("xxx").unwrap();

    // let res = api::read_clipboard_data();
    // println!("{}", res);
}
