# Screenrecord

```bash
sudo pacman -S slurp grim satty ffmpeg
```

Create `~/.local/bin/screenrecord-start`:

```sh
#!/usr/bin/env bash
mkdir -p ~/Videos/Recordings
GEOM=$(slurp) || exit 1
FILENAME=~/Videos/Recordings/$(date +%y%m%d_%Hh%Mm%Ss)_recording.mp4
wf-recorder -g "$GEOM" -f "$FILENAME" &
echo $! > /tmp/wf-recorder.pid
notify-send "Recording started" "Saving to $FILENAME"
```

Create `~/.local/bin/screenrecord-stop`:

```sh
#!/usr/bin/env bash
if [ -f /tmp/wf-recorder.pid ]; then
  kill -INT $(cat /tmp/wf-recorder.pid)
  rm /tmp/wf-recorder.pid
  notify-send "Recording stopped" "Saved to ~/Videos/Recordings/"
else
  notify-send "No recording in progress" "No PID file found"
fi
```

Make them executable:

```bash
sudo chmod +x screenrecord-start screenrecord-stop
```

## MangoWC Keybind

```conf
# Screenshots and recording
bind=SUPER,P,spawn_shell,mkdir -p ~/Pictures/Screenshots && grim -g "$(slurp)" - | satty --filename -

bind=SUPER+SHIFT,P,spawn,~/.local/bin/screenrecord-start
bind=SUPER+SHIFT,O,spawn,~/.local/bin/screenrecord-stop
```
