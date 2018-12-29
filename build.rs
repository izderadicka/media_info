use std::process;
use std::path;

fn main() {

    let ffmpeg_dir = path::Path::new("ffmpeg-4.1");
    if ! ffmpeg_dir.exists() {
        let rc =  process::Command::new("./build_ffmpeg.sh").status().expect("cannot run build script");
        if ! rc.success() {
            panic!("build script failed with {:?}", rc.code());
        }

    }

    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-search=native=ffmpeg-4.1/libavformat");
    println!("cargo:rustc-link-search=native=ffmpeg-4.1/libavutil");
    println!("cargo:rustc-link-search=native=ffmpeg-4.1/libavcodec");
}
