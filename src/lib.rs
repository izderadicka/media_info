#[macro_use]
extern crate quick_error;
use std::ffi::CString;
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

    }
}

// struct AvDictionary {
//     pub dictionary: *mut ffi::AVDictionary,
// }

// impl Drop for AvDictionary {
//     fn drop(&mut self) {
//         unsafe {
//             ffi::av_dict_free(&mut self.dictionary)
//         }
//     }
// }

// impl AvDictionary {
//     fn new() -> AvDictionary {
//         AvDictionary {
//             dictionary: ptr::null_mut(),
//         }
//     }

//     fn set(&mut self, key: &str, value: &str) {
//         unsafe {
//             let key = CString::new(key.as_bytes()).unwrap();
//             let value = CString::new(value.as_bytes()).unwrap();
//             assert!(ffi::av_dict_set(&mut self.dictionary, key.as_ptr(), value.as_ptr(), 0) >= 0);
//         }
//     }
// }

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
    unsafe {
        ffi::avformat_version()
    }
}

pub struct MediaFile {
    ctx: *mut ffi::AVFormatContext
}

impl MediaFile {
    pub fn open<S:AsRef<str>>(fname: S) -> Result<Self> {
        unsafe {
        let mut ctx = ffi::avformat_alloc_context();
        assert!(ctx as usize > 0);
        let name = CString::new(fname.as_ref()).unwrap();
        let ret = ffi::avformat_open_input(&mut ctx, name.as_ptr(), ptr::null_mut(), ptr::null_mut());
        check_ret(ret)?;
        if ctx as usize == 0 {
            return Err(Error::AllocationError)
        }
        let ret = ffi::avformat_find_stream_info(ctx, ptr::null_mut());
        check_ret(ret)?;

        // --------------------------------------------------------
        Ok(MediaFile {ctx})
        }
    }

    /// Duration in ms
    pub fn duration(&self) -> u64 {
        
        let d = unsafe {(*self.ctx).duration}/ 1_000;
        if d <0 {
            return 0
        } else {
            d as u64
        
        }
    }

    /// bitrate in kbps 
    pub fn bitrate(&self) -> u32 {
        let b = unsafe {
        (*self.ctx).bit_rate
        }/ 1000;

        if b < 0 {
            return 0
        } else {
            b as u32
        }

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
    fn test_duration_and_bitrate() {
        init();
        let mf = MediaFile::open("test_files/test.mp3").unwrap();
        let dur = mf.duration() ;
        let br = mf.bitrate();
        println!("Duration {}, bitrate {}", dur, br);
        assert!(dur/ 1_000  == 283 );
        assert!(br == 192);
        unsafe {
        ffi::av_dump_format(mf.ctx, 0, ptr::null(), 0);
        }
        
    }
}
