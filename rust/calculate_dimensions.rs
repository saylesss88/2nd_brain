pub fn calculate_dimensions(&self, orig_w: u32, orig_h: u32) -> (u32, u32) {
    const MAX_SAFE: u32 = 16_384;

    fn fit_preserving_aspect(
        orig_w: u32,
        orig_h: u32,
        max_w: u32,
        max_h: u32,
        cell_w: f64,
        cell_h: f64,
    ) -> (u32, u32) {
        let aspect = f64::from(orig_h) / f64::from(orig_w);
        let cell_aspect = cell_h / cell_w;

        #[expect(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "Terminal dimension math uses bounded float-to-u32 conversions"
        )]
        {
            let h_from_w = (f64::from(max_w) * aspect * cell_aspect).ceil() as u32;

            if h_from_w <= max_h {
                (max_w, h_from_w.max(1))
            } else {
                let w_from_h = (f64::from(max_h) / (aspect * cell_aspect)).floor() as u32;
                (w_from_h.max(1), max_h)
            }
        }
    }

    fn mode_cell_dims(mode: CharsetMode, unicode_full: bool, wide: bool) -> (f64, f64) {
        match mode {
            CharsetMode::Braille => (2.0, 4.0),
            CharsetMode::Unicode if unicode_full => (2.0, 1.0),
            CharsetMode::Sixel => (8.0, 16.0),

            // Wide terminal glyphs: 2 columns × 1 row
            _ if wide => (2.0, 1.0),

            // Narrow text modes: 1 column × 1 row
            CharsetMode::Ascii | CharsetMode::Fade => (1.0, 1.0),

            // Fallback legacy behavior
            _ => (1.0, 1.0),
        }
    }

    let (term_w, term_h) = get_terminal_size();

    let (max_w, max_h) = if term_w > 0 && term_h > 0 {
        match self.charset() {
            CharsetMode::Braille => (term_w * 2, term_h * 4),
            CharsetMode::Unicode if self.style().full => (term_w / 2, term_h),
            CharsetMode::Sixel => (term_w * 8, term_h * 16),
            _ => (term_w.saturating_sub(2), term_h),
        }
    } else {
        (80, 40)
    };

    let (cell_w, cell_h) = mode_cell_dims(self.charset(), self.style().full, self.style().wide);

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "Terminal scaling math uses intentional bounded conversions"
    )]
    let (render_w, render_h) = self.width().map_or_else(
        || {
            if self.filter() == FilterType::Nearest && !self.style().wide && orig_w < 120 {
                let scale_w = (f64::from(max_w) / f64::from(orig_w)).floor();
                let scale_h = (f64::from(max_h) / f64::from(orig_h)).floor();
                let scale = scale_w.min(scale_h).max(1.0);

                (
                    (f64::from(orig_w) * scale) as u32,
                    (f64::from(orig_h) * scale) as u32,
                )
            } else {
                fit_preserving_aspect(orig_w, orig_h, max_w, max_h, cell_w, cell_h)
            }
        },
        |tw| {
            let aspect = f64::from(orig_h) / f64::from(orig_w);

            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "Width-derived height is intentionally rounded upward"
            )]
            {
                let h = (f64::from(tw) * aspect * (cell_h / cell_w)).ceil() as u32;
                (tw, h)
            }
        },
    );

    (render_w.clamp(1, MAX_SAFE), render_h.clamp(1, MAX_SAFE))
}
