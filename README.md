# media_info

**moved to audioserve repo**

Simple Rust binding to libavformat (from ffmpeg progject) to extract duration and bitrate from media file.

See [example code](examples/media_info.rs) for usage.

## requirements

Under Linux you'll need regular build environment (gcc, make ...) and nasm/yasm and zlib and bz2lib to build this crate.

Build process requires wget and access to Internet to get ffmpeg-4.1.

