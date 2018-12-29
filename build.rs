fn main() {
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rustc-link-search=native=ffmpeg-4.1/libavformat");
    println!("cargo:rustc-link-search=native=ffmpeg-4.1/libavutil");
    println!("cargo:rustc-link-search=native=ffmpeg-4.1/libavcodec");
}
