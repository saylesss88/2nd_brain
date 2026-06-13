# Mpv

Install:

```bash
sudo pacman -S mpv
```

```conf
# Hardware decoding
hwdec=vaapi
hwdec-codecs=h264,hevc,vp9,vc1

# Video output
vo=gpu
gpu-api=vulkan
gpu-context=auto

# Scaling
scale=spline36
cscale=spline36
# scale=ewa_lanczos
# cscale=ewa_lanczos

# Debanding
deband=yes
deband-grain=8
deband-range=16

# Audio
audio-normalize-downmix=yes
audio-device=auto

# Frame/sync
framedrop=vo
video-sync=display-resample
interpolation=yes
tscale=oversample

# Cache
cache=yes
cache-secs=10
demuxer-readahead-secs=10
demuxer-max-bytes=20MiB

# YouTube
ytdl-format=bestvideo[height<=1080]+bestaudio/best

# Subtitles
sub-auto=fuzzy
# sub-font="Arial"
sub-font="IosevkaNerdFontMono-SemiBold"
sub-font-size=32
sub-color="#FFFFFF"
sub-shadow-offset=1.5
sub-blur=0.5

# Misc
force-window=yes

[low-latency]
video-sync=display-resample
interpolation=no
cache-secs=3
demuxer-readahead-secs=3

[4k]
scale=ewa_lanczos
cscale=ewa_lanczos
deband-grain=16
deband-range=32
cache-secs=20
demuxer-max-bytes=40MiB
ytdl-format=bestvideo[height<=1080][vcodec!=av01]+bestaudio/best
```
