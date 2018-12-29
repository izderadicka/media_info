#! /bin/bash
bindgen wrapper.h -- -I ffmpeg-4.1 > src/ffi.rs