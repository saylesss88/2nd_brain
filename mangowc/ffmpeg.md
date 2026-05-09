
```bash
ffmpeg -i 260426_17h19m18s_recording.mp4 -vf "fps=10,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=64[p];[s1][p]paletteuse" -loop 0 output2.gif
```
