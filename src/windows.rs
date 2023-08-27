use std::fmt::Display;
use std::io::Cursor;
use std::mem::{size_of, self};
use std::ptr;
use std::{ffi::CStr, fs};
use std::os::raw::c_char;
use clipboard_win::raw::{set_without_clear};
use clipboard_win::{SysResult, set_clipboard, get_clipboard_string, SystemError};
use clipboard_win::{Clipboard, formats, Getter, EnumFormats, register_format, raw::get_clipboard_data, Setter};
use image::{ImageFormat, DynamicImage};
use winapi::shared::minwindef::DWORD;
use winapi::shared::winerror::ERROR_INCORRECT_SIZE;
use winapi::um::wingdi::{BITMAPV5HEADER, BI_BITFIELDS, LCS_sRGB, LCS_GM_IMAGES, BITMAPFILEHEADER, BITMAPINFOHEADER, CreateDIBitmap, CBM_INIT, BITMAPINFO, DIB_RGB_COLORS};
use winapi::um::winnt::LONG;
use winapi::um::winuser::{GetDC, SetClipboardData, ReleaseDC};

pub fn set_bitmap_without_clear(data: &[u8]) -> SysResult<()> {
    const FILE_HEADER_LEN: usize = mem::size_of::<BITMAPFILEHEADER>();
    const INFO_HEADER_LEN: usize = mem::size_of::<BITMAPINFOHEADER>();

    if data.len() <= (FILE_HEADER_LEN + INFO_HEADER_LEN) {
        return Err(SystemError::new(ERROR_INCORRECT_SIZE as _));
    }

    let mut file_header = mem::MaybeUninit::<BITMAPFILEHEADER>::uninit();
    let mut info_header = mem::MaybeUninit::<BITMAPINFOHEADER>::uninit();

    let (file_header, info_header) = unsafe {
        ptr::copy_nonoverlapping(data.as_ptr(), file_header.as_mut_ptr() as _, FILE_HEADER_LEN);
        ptr::copy_nonoverlapping(data.as_ptr().add(FILE_HEADER_LEN), info_header.as_mut_ptr() as _, INFO_HEADER_LEN);
        (file_header.assume_init(), info_header.assume_init())
    };

    if data.len() <= file_header.bfOffBits as usize {
        return Err(SystemError::new(ERROR_INCORRECT_SIZE as _));
    }

    let bitmap = &data[file_header.bfOffBits as _..];

    if bitmap.len() < info_header.biSizeImage as usize {
        return Err(SystemError::new(ERROR_INCORRECT_SIZE as _));
    }

    let dc = unsafe { GetDC(ptr::null_mut()) };

    let handle = unsafe {
        CreateDIBitmap(dc, &info_header as _, CBM_INIT, bitmap.as_ptr() as _, &info_header as *const _ as *const BITMAPINFO, DIB_RGB_COLORS)
    };

    unsafe {
        ReleaseDC(ptr::null_mut(), dc);
    }
    
    if handle.is_null() {
        return Err(SystemError::last());
    }

    if unsafe { SetClipboardData(formats::CF_BITMAP, handle as _).is_null() } {
        return Err(SystemError::last());
    }

    Ok(())
}
