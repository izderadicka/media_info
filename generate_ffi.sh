#! /bin/bash
bindgen \
--no-doc-comments \
--whitelist-type AVFormatContext \
--whitelist-function av_log_set_level \
--whitelist-function av_register_all \
--whitelist-function avformat_version \
--whitelist-function avformat_alloc_context \
--whitelist-function avformat_open_input \
--whitelist-function avformat_find_stream_info \
--whitelist-function avformat_close_input \
--whitelist-function av_dump_format \
--whitelist-var AV_LOG_QUIET \
wrapper.h -- -I ffmpeg-4.1 \
> src/ffi.rs