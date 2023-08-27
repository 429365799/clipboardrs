use std::fmt::Display;
use std::io::Cursor;
use std::{ffi::CStr, fs};
use std::os::raw::c_char;
use clipboard_win::raw::{set_without_clear};
use clipboard_win::{SysResult, set_clipboard, get_clipboard_string};
use clipboard_win::{Clipboard, formats, Getter, EnumFormats, register_format, raw::get_clipboard_data};
use image::{ImageFormat, DynamicImage};

// use crate::windows;

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

pub fn write_image(img: DynamicImage) -> SysResult<()> {
    let _clip = open_clipboard();
    let cf_html: u32 = register_format("HTML Format").unwrap().into();
    let mut buf: Vec<u8> = Vec::new();

    img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Bmp).expect("图片转换失败");
    // formats::Bitmap::write_clipboard(&formats::Bitmap, &mut buf);
    // formats::Unicode::write_clipboard(&formats::Unicode, &String::from("xdf"))
    // let rd = formats::RawData(formats::CF_TEXT);
    // rd.write_clipboard(&"xdfsfsadf").unwrap();

    // let rd1 = formats::RawData(formats::CF_BITMAP);
    // rd1.write_clipboard(&buf)

    // set_clipboard(formats::Bitmap, &buf)?;
    // close().unwrap();

    // Clipboard::new_attempts(10).expect("打开剪贴板失败");
    if let Err(e) = set_without_clear(formats::CF_TEXT, b"sdfsdf") {
        println!("{:?}", e);
    } else {
        println!("No Error???")
    }
    if let Err(e) = set_without_clear(cf_html, wrap_html("<div>asdfasdfasdf</div>").as_bytes()) {
        println!("{:?}", e);
    } else {
        println!("No Error???")
    }

    // if let Err(e) = set_without_clear(formats::CF_DIBV5, &buf) {
    //     println!("{:?}", e);
    // } else {
    //     println!("No Error???")
    // }
    // let b = formats::Bitmap {};
    // b.write_clipboard(&buf).unwrap();
    
    // set_bitmap_without_clear(&buf)?;
    
    // let mut ctx = arboard::Clipboard::new().unwrap();
    // ctx.set_html(wrap_html("<div>ASDSSDF</div"), None).unwrap();
    // ctx.set_text("text").unwrap();

    Ok(())
}

pub fn read_html() -> String {
    let _clip = open_clipboard();
    let cf_html: u32 = register_format("HTML Format").unwrap().into();
    read_custom_format_str(cf_html)
}


fn wrap_html(ctn: &str) -> String {
	let h_version = "Version:0.9";
	let h_start_html = "\r\nStartHTML:";
	let h_end_html = "\r\nEndHTML:";
	let h_start_frag = "\r\nStartFragment:";
	let h_end_frag = "\r\nEndFragment:";
	let c_start_frag = "\r\n<html>\r\n<body>\r\n<!--StartFragment-->\r\n";
	let c_end_frag = "\r\n<!--EndFragment-->\r\n</body>\r\n</html>";
	let h_len = h_version.len()
		+ h_start_html.len()
		+ 10 + h_end_html.len()
		+ 10 + h_start_frag.len()
		+ 10 + h_end_frag.len()
		+ 10;
	let n_start_html = h_len + 2;
	let n_start_frag = h_len + c_start_frag.len();
	let n_end_frag = n_start_frag + ctn.len();
	let n_end_html = n_end_frag + c_end_frag.len();
	format!(
		"{}{}{:010}{}{:010}{}{:010}{}{:010}{}{}{}",
		h_version,
		h_start_html,
		n_start_html,
		h_end_html,
		n_end_html,
		h_start_frag,
		n_start_frag,
		h_end_frag,
		n_end_frag,
		c_start_frag,
		ctn,
		c_end_frag,
	)
}

pub fn write_text(text: &str) -> SysResult<()> {
    let _clip = open_clipboard();
    set_clipboard(formats::Unicode, &text)
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
    pub text: Option<String>,
    pub html: Option<String>,
    pub image: Option<DynamicImage>,
    pub files: Option<Vec<ClipboardFile>>,
}

impl Display for ClipboardData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();

        str.push_str("{");
        
        if self.text != None {
            str.push_str("\ntext: ");
            str.push_str(self.text.clone().unwrap().as_str());
            str.push_str(",");
        }
        if self.html != None {
            str.push_str("\nhtml: ");
            str.push_str(self.html.clone().unwrap().as_str());
            str.push_str(",");
        }
        if self.image != None {
            str.push_str("\nimage: ");
            let img = self.image.clone().unwrap(); 
            str.push_str(format!("Image (width: {}, height, {})", img.width(), img.height()).as_str());
            str.push_str(",");
        }
        if self.files != None {
            str.push_str("\nfiles: ");
            str.push_str(format!("{:?}", self.files.clone().unwrap()).as_str());
            str.push_str(",");
        }

        str.push_str("\n}");

        write!(f, "{}", str)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClipboardFile {
    pub path: String,
    pub size: u64,
}

pub fn read_clipboard_data() -> ClipboardData {
    let _clip = Clipboard::new_attempts(10).expect("打开剪贴板失败");
    let ef = EnumFormats::new();
    let cf_html: u32 = register_format("HTML Format").unwrap().into();
    let mut result = ClipboardData {
        text: None,
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
            let out_text = get_clipboard_string().expect("读取失败");
            result.text = Some(out_text);
        } else if x == cf_html {
            let html = read_custom_format_str(cf_html);
            result.html = Some(html);
        } else if x == formats::CF_HDROP {
            let mut file_contents = Vec::new();
            let mut file_list: Vec<ClipboardFile> = Vec::new(); 
            formats::FileList::read_clipboard(&formats::FileList, &mut file_contents).expect("读取文件失败");
            for item in file_contents {
                let meta = fs::metadata(item.clone()).unwrap();
                file_list.push(ClipboardFile { path: item, size: meta.len() })
            }
            result.files = Some(file_list);
        }
        
    });

    result
}
