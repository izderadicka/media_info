#[macro_use]
extern crate quick_error;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        AVError(code:i32) {
            display("libav error code {}", code)
        }
        AllocationError {
            display("memory allocation error - maybe full memory")
        }
        InvalidString(err: std::str::Utf8Error) {
            display("UTF8 error: {}", err)
            from()
        }

    }
}

// fn string_from_ptr(ptr: *const c_char) -> Result<Option<String>> {
//     if ptr.is_null() {
//         Ok(None)
//     } else {
//         unsafe { Ok(Some(CStr::from_ptr(ptr).to_str()?.to_owned())) }
//     }
// }

fn string_from_ptr_lossy(ptr: *const c_char) -> String {
    if ptr.is_null() {
        "".into()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into() }
    }
}

struct Dictionary {
    pub dic: *mut ffi::AVDictionary,
}

impl Dictionary {
    fn new(dic: *mut ffi::AVDictionary) -> Self {
        Dictionary { dic }
    }

    fn get<S: AsRef<str>>(&self, key: S) -> Option<String> {
        if self.dic.is_null() {
            return None;
        }
        let cs = CString::new(key.as_ref()).expect("zero byte in key");
        unsafe {
            let res = ffi::av_dict_get(self.dic, cs.as_ptr(), ptr::null(), 0);
            if res.is_null() {
                return None;
            }
            return Some(string_from_ptr_lossy((*res).value));
        }
    }

    fn get_all(&self) -> HashMap<String, String> {
        let empty = CString::new("").unwrap();
        let mut map = HashMap::new();
        let mut prev = ptr::null();
        loop {
            unsafe {
                let current = ffi::av_dict_get(self.dic, empty.as_ptr(), prev, ffi::AV_DICT_IGNORE_SUFFIX as i32);
                if current.is_null() {
                    break;
                } else {
                    let key = string_from_ptr_lossy((*current).key);
                    let value = string_from_ptr_lossy((*current).value);
                    map.insert(key, value);
                    prev = current;
                }
            }
        }

        map
    }
}

pub type Result<T> = std::result::Result<T, Error>;

fn check_ret(res: i32) -> Result<()> {
    if res == 0 {
        Ok(())
    } else {
        Err(Error::AVError(res))
    }
}

pub fn init() {
    unsafe {
        ffi::av_log_set_level(ffi::AV_LOG_QUIET);
        ffi::av_register_all()
    }
}

pub fn version() -> u32 {
    unsafe { ffi::avformat_version() }
}

pub struct MediaFile {
    ctx: *mut ffi::AVFormatContext,
    meta: Dictionary,
}

macro_rules! meta_methods {
    ($self:ident $( $name:ident )+) => {
        $(
        pub fn $name(&$self) -> Option<String> {
        $self.get_meta(stringify!($name))
        }
        )+
    };
}

impl MediaFile {
    pub fn open<S: AsRef<str>>(fname: S) -> Result<Self> {
        unsafe {
            let mut ctx = ffi::avformat_alloc_context();
            assert!(ctx as usize > 0);
            //(*ctx).probesize = 5*1024*1024*1024;
            let name = CString::new(fname.as_ref()).unwrap();
            let ret =
                ffi::avformat_open_input(&mut ctx, name.as_ptr(), ptr::null_mut(), ptr::null_mut());
            check_ret(ret)?;
            if ctx.is_null() {
                return Err(Error::AllocationError);
            }
            let ret = ffi::avformat_find_stream_info(ctx, ptr::null_mut());
            check_ret(ret)?;

            // --------------------------------------------------------
            Ok(MediaFile {
                ctx,
                meta: Dictionary::new((*ctx).metadata),
            })
        }
    }

    /// Duration in ms
    pub fn duration(&self) -> u64 {
        let d = unsafe { (*self.ctx).duration } / 1_000;
        if d < 0 {
            return 0;
        } else {
            d as u64
        }
    }

    /// bitrate in kbps
    pub fn bitrate(&self) -> u32 {
        let b = unsafe { (*self.ctx).bit_rate } / 1000;

        if b < 0 {
            return 0;
        } else {
            b as u32
        }
    }
    meta_methods!(self title album artist composer genre track  );

    pub fn get_meta<S: AsRef<str>>(&self, key: S) -> Option<String> {
        self.meta
            .get(&key)
            //.or_else(|| self.meta.get(key.as_ref().to_uppercase()))
    }

    pub fn get_all_meta(&self) -> HashMap<String,String> {
        self.meta.get_all()
    }
}

impl Drop for MediaFile {
    fn drop(&mut self) {
        unsafe {
            ffi::avformat_close_input(&mut self.ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_meta() {
        init();
        let mf = MediaFile::open("test_files/test.mp3").unwrap();
        let dur = mf.duration();
        let br = mf.bitrate();
        println!("Duration {}, bitrate {}", dur, br);
        assert!(dur / 1_000 == 283);
        assert!(br == 192);
        assert_eq!("00.uvod", mf.title().unwrap());
        assert_eq!("Stoparuv pruvodce po galaxii", mf.album().unwrap());
        assert_eq!("Vojtěch Dyk", mf.artist().unwrap());
        assert_eq!("Adam Douglas", mf.composer().unwrap());
        assert!(mf.get_meta("usak").is_none());
        let meta = mf.get_all_meta();
        assert_eq!("00.uvod", meta.get("title").unwrap());
        unsafe {
            ffi::av_dump_format(mf.ctx, 0, ptr::null(), 0);
        }
    }
}
