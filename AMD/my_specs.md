# AMD and MPV

Your stack breakdown:

- radeonsi = Mesa OpenGL driver for your iGPU ✅

- renoir = Ryzen 4000/5000/7000 iGPU codename ✅

- ACO = AMD Compiler (newer, faster than LLVMpipe) ✅

- DRM 3.64 = Kernel amdgpu driver ✅

- 6.19.6 = Your latest kernel ✅

✅ You're 100% ready:

- VA-API: H.264 decode/encode confirmed (wf-recorder videos)

- OpenGL: radeonsi + ACO confirmed (mpv GPU rendering)

- Terminal playback: mpv recording.mp4 → tct crisp text

- GUI acceleration: `mpv --profile=amd-vaapi video.mp4` → full hardware
