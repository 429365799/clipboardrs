use std::fs::File;
use std::io::{Read, BufReader, BufRead, Cursor};
use std::path::Path;
use std::{ffi::CStr, fs};
use std::os::raw::c_char;
use std::prelude;

use clipboard_win::formats::FileList;
use clipboard_win::{Clipboard, formats, Getter, EnumFormats, register_format, raw::get_clipboard_data, Setter};
use image::{ImageFormat, DynamicImage};

fn open_clipboard() -> Clipboard {
    Clipboard::new_attempts(10).expect("打开剪贴板失败")
}

pub enum CustomFormat {
    Html,
    OemText,
}

pub fn read_image() -> image::DynamicImage {
    let _clip = open_clipboard();
    let mut out_image: Vec<u8> = Vec::new();

    formats::Bitmap::read_clipboard(&formats::Bitmap, &mut out_image).expect("读取图片失败");
    image::load_from_memory_with_format(&out_image, ImageFormat::Bmp).expect("图片转换失败")
}

pub fn write_image(img: DynamicImage) {
    let _clip = open_clipboard();
    let mut buf = Vec::new();

    // let img: DynamicImage = image::open("D:/workspace/ws_rust/clipboardrs/src/img.jpg").unwrap();
    
    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Bmp).expect("图片转换失败");

    formats::Bitmap::write_clipboard(&formats::Bitmap, &mut buf).expect("图片写入剪贴板失败")
}

pub fn read_html() -> String {
    let _clip = open_clipboard();
    let cf_html: u32 = register_format("HTML Format").unwrap().into();
    read_custom_format_str(cf_html)
}

fn read_custom_format_str(format: u32) -> String {
    let ptr = get_clipboard_data(format).expect(format!("读取{}失败！", format).as_str());
    let char_ptr: *const c_char = ptr.cast().as_ptr();
    let cstr = unsafe {
        CStr::from_ptr(char_ptr)
    };
    
    cstr.to_string_lossy().into_owned()
}

pub struct ClipboardData {
    text: Option<String>,
    oem_text: Option<String>,
    html: Option<String>,
    image: Option<DynamicImage>,
    files: Option<ClipboardFileList>,
}

pub struct ClipboardFileList {
    
}

pub fn read_clipboard_data() -> ClipboardData {
    let _clip = Clipboard::new_attempts(10).expect("打开剪贴板失败");
    println!("剪贴板打开成功！");
    let ef = EnumFormats::new();
    let cf_html: u32 = register_format("HTML Format").unwrap().into();
    let oem_text: u32 = register_format("OEM Text").unwrap().into();
    let mut result = ClipboardData {
        text: None,
        oem_text: None,
        html: None,
        image: None,
        files: None,
    };

    ef.for_each(|x| {
        if x == formats::CF_BITMAP {
            let img = read_image();
            result.image = Some(img);
            // img.save("./out.bmp").expect("写入图片失败");
        } else if x == formats::CF_TEXT {
            let mut out_text: String = String::new();
            formats::Unicode::read_clipboard(&formats::Unicode, &mut out_text).expect("读取失败");
            // println!("TEXT: {}", out_text);
            result.text = Some(out_text);
        } else if x == cf_html {
            let html = read_custom_format_str(cf_html);
            println!("HTML: {:?}", html);
            result.html = Some(html);
        } else if x == oem_text {
            let oem = read_custom_format_str(oem_text);
            println!("OEM Text: {:?}", oem);
            result.oem_text = Some(oem);
        } else if x == formats::CF_HDROP {
            let mut file_contents = Vec::new();
            let file_count = formats::FileList::read_clipboard(&formats::FileList, &mut file_contents).expect("读取文件失败");
            println!("FILE: count: {:?}  list: {:?}", file_count, file_contents);
        }
    });

    result
}
