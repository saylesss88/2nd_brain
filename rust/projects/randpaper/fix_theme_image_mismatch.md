Fix theme/image mismatch Right now you call update_theme_file(img) on one random
image, then (for both renderers) you pick different random images per monitor—so
the generated theme often won’t match any displayed wallpaper. Instead, pick the
per-monitor images first, then generate the theme from a deterministic choice
(commonly: the first/primary monitor’s wallpaper).

One cycle function Refactor the body of the loop into something like
cycle_once(...) that (1) selects images, (2) updates the theme once, (3) reloads
Waybar, (4) applies wallpapers via the chosen renderer.

```rs
async fn reload_waybar_sigusr2() {
    // Ignore failure if waybar isn't running.
    let _ = Command::new("pkill")
        .arg("-SIGUSR2")
        .arg("waybar")
        .status()
        .await;
}

pub async fn run_loop<B: Backend>(cli: Cli, backend: B) -> anyhow::Result<()> {
    let cache = WallpaperCache::new(&cli.wallpaper_dir)?;
    let period: Duration =
        parse_duration::parse(&cli.time).map_err(|e| anyhow::anyhow!("invalid duration: {e}"))?;

    let mut current_swaybg: Option<Child> = None;

    let swww_bin = if cli.renderer == RendererType::Swww {
        detect_swww_binary().await
    } else {
        String::new()
    };

    if cli.renderer == RendererType::Swww {
        let daemon_cmd = format!("{swww_bin}-daemon");
        let _ = Command::new(&daemon_cmd).spawn();
        sleep(Duration::from_millis(500)).await;
    }

    let mut sig_usr1 = signal(SignalKind::user_defined1())?;

    let mut cycle_once = |monitors: Vec<String>| async {
        // 1) Pick images *once* per output
        let chosen: Vec<(String, PathBuf)> = monitors
            .into_iter()
            .map(|m| (m, cache.pick_random().to_path_buf()))
            .collect();

        // 2) Theme from the first output’s image (or whichever rule you prefer)
        if let Some((_, theme_img)) = chosen.first() {
            let _ = update_theme_file(theme_img);
            reload_waybar_sigusr2().await;
        }

        // 3) Apply wallpapers
        match cli.renderer {
            RendererType::Swaybg => {
                // Build args from our chosen set (no extra random picks)
                let mut args = Vec::new();
                for (monitor, img) in &chosen {
                    let Ok(abs_path) = img.canonicalize() else { continue };
                    args.extend([
                        "-o".to_string(), monitor.clone(),
                        "-m".to_string(), "fill".to_string(),
                        "-i".to_string(), abs_path.to_string_lossy().to_string(),
                    ]);
                }

                if !args.is_empty() {
                    if let Some(mut child) = current_swaybg.take() {
                        let _ = child.kill().await;
                        let _ = child.wait().await;
                    }
                    if let Ok(child) = Command::new("swaybg")
                        .args(&args)
                        .kill_on_drop(true)
                        .spawn()
                    {
                        current_swaybg = Some(child);
                    }
                }
            }
            RendererType::Swww => {
                let step = cli.transition_step.to_string();
                let fps = cli.transition_fps.to_string();

                for (monitor, img) in &chosen {
                    let out = Command::new(&swww_bin)
                        .arg("img")
                        .arg(img)
                        .arg("-o")
                        .arg(monitor)
                        .arg("--transition-type")
                        .arg(&cli.transition_type)
                        .arg("--transition-step")
                        .arg(&step)
                        .arg("--transition-fps")
                        .arg(&fps)
                        .output()
                        .await
                        .with_context(|| format!("failed to run {swww_bin}"))?;

                    if !out.status.success() {
                        anyhow::bail!(
                            "{swww_bin} failed: {}",
                            String::from_utf8_lossy(&out.stderr)
                        );
                    }
                }
            }
        }

        anyhow::Ok(())
    };

    loop {
        let monitors = match backend.get_active_monitors().await {
            Ok(m) => m,
            Err(e) => {
                log::error!("Failed to get monitors: {e}. Retrying in 5s...");
                sleep(Duration::from_secs(5)).await;
                continue;
            }
        };

        // Timer vs. SIGUSR1 both trigger the exact same cycle path
        tokio::select! {
            () = async {
                cycle_once(monitors).await?;
                sleep(period).await;
                anyhow::Ok::<(), anyhow::Error>(())
            } => {}
            _ = sig_usr1.recv() => {
                log::info!("Received skip signal (SIGUSR1). Cycling wallpaper immediately.");
                cycle_once(monitors).await?;
            }
        }
    }
}
```

Waybar reload behavior Waybar’s default on-sigusr2 action is reload, and it can
be configured via on-sigusr1 / on-sigusr2 in the config. ​ So your “generate
theme then SIGUSR2” approach is aligned with how Waybar expects to be reloaded.
​

“Cycle through” vs random If you literally want to cycle through wallpapers (not
random), add a small “cycler” that keeps a shuffled Vec<PathBuf> + index
(reshuffle when you hit the end), and use cycler.next() instead of
cache.pick_random(). If you show me WallpaperCache, I can suggest the cleanest
place to put that state (inside the cache vs. a separate WallpaperCycler stored
in run_loop).
