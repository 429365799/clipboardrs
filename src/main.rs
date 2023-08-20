use std::ffi::CStr;
use std::os::raw::c_char;

use clipboard_win::{Clipboard, formats, Getter, EnumFormats, register_format, raw::get_clipboard_data, is_format_avail};
use image::ImageFormat;

pub mod api;

fn main() {
    let img = image::open("D:/workspace/ws_rust/clipboardrs/src/img.jpg").unwrap();
    api::write_image(img);
    return;

    // 这里必须用一个变量保存，它是剪贴板的句柄
    let _clip = Clipboard::new_attempts(10).expect("打开剪贴板失败");
    println!("剪贴板打开成功！");

    let ef = EnumFormats::new();
    let cf_html: u32 = register_format("HTML Format").unwrap().into();
    let oem_text: u32 = register_format("OEM Text").unwrap().into();

    is_format_avail(cf_html);
    
    ef.for_each(|x| {
        if x == formats::CF_BITMAP {
            let img = api::read_image();
            img.save("./out.bmp").expect("写入图片失败");
        } else if x == formats::CF_TEXT {
            let mut out_text = String::new();
            formats::Unicode::read_clipboard(&formats::Unicode, &mut out_text).expect("Read！");
            println!("TEXT: {}", out_text);
        } else if x == cf_html {
            let html = read_custom_format_str(cf_html);
            println!("HTML: {:?}", html);
        } else if x == oem_text {
            let oem = read_custom_format_str(oem_text);
            println!("OEM Text: {:?}", oem);
        } else if x == formats::CF_HDROP {
            let mut file_contents = Vec::new();
            let file_count = formats::FileList::read_clipboard(&formats::FileList, &mut file_contents).expect("读取文件失败");
            println!("FILE: count: {:?}  list: {:?}", file_count, file_contents);
        }
    });
}

fn read_custom_format_str(format: u32) -> String {
    let ptr = get_clipboard_data(format).expect(format!("读取{}失败！", format).as_str());
    let char_ptr: *const c_char = ptr.cast().as_ptr();
    let cstr = unsafe {
        CStr::from_ptr(char_ptr)
    };
    
    cstr.to_string_lossy().into_owned()
}
