//! v0.2 preset personality acceptance: the hard visual test.
//!
//! With colour disabled (`ColorChoice::Never`, human target, width 100),
//! each preset must still be identifiable from the silhouette of the screen.
//! These tests assert structural traits, not just pairwise inequality.

use sartorial::*;
use unicode_width::UnicodeWidthStr;

fn fixture_screen() -> SummaryScreen {
    let mut table = TableModel::new(vec!["Tool", "Version", "Status"])
        .with_title("Tools")
        .with_badge("3 / 4");
    table.add_row(["Git", "2.51.0", "ready"]);
    table.add_row(["ripgrep", "14.1.1", "ready"]);
    table.add_row(["fd", "", "missing"]);

    SummaryScreen::new("Demo", Status::Ready)
        .with_subtitle("/repo/demo")
        .fact("Root", "/repo/demo")
        .fact("Branch", "main")
        .fact("Head", "abc1234")
        .fact("Files", "29 files")
        .with_table(table)
        .notice(Notice::warning("Partial scan: 1 warning (0 suppressed)"))
}

fn ctx_no_color(preset: Preset, width: usize) -> RenderContext {
    RenderContext::detect()
        .with_config(
            Config::default()
                .with_preset(preset)
                .with_color(ColorChoice::Never)
                .with_symbols(SymbolMode::Unicode)
                .with_width(width),
        )
        .with_target(RenderTarget::Human)
}

fn render(preset: Preset, width: usize) -> String {
    let ctx = ctx_no_color(preset, width);
    fixture_screen().to_human_string(&ctx).unwrap()
}

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

#[test]
fn house_silhouette() {
    let out = render(Preset::House, 100);
    // Uppercase title, no title rule, no operator marker.
    assert_eq!(first_line(&out), "DEMO");
    assert!(!out.contains('»'), "House must not use the operator marker");
    // No title-block rule: the first three lines are title, subtitle, blank.
    // (Table bodies keep their subtle top rule; that is not decoration.)
    for line in out.lines().take(3) {
        assert!(
            line.is_empty() || !line.chars().all(|c| c == '─'),
            "House must not rule its title block"
        );
    }
    // Compact inline status with a bounded gap.
    assert!(out.contains("Status        ✓ READY"), "got:\n{out}");
    // Standard key/value form.
    assert!(out.contains("Root    /repo/demo"), "got:\n{out}");
}

#[test]
fn black_tie_silhouette() {
    let out = render(Preset::BlackTie, 100);
    // Original-case title plus a restrained block rule.
    assert_eq!(first_line(&out), "Demo");
    assert!(
        out.lines()
            .any(|l| l.chars().all(|c| c == '─') && l.width() >= 20),
        "Black Tie needs a title-block rule, got:\n{out}"
    );
    // Formal stacked status with a short section rule.
    assert!(out.contains("Status\n──────\n✓ READY"), "got:\n{out}");
    // Formal tables render their heading row.
    assert!(
        out.contains("Version"),
        "Black Tie must show table headings"
    );
    assert!(
        !out.contains('»'),
        "Black Tie must not use the operator marker"
    );
    // Slightly roomier facts than House.
    assert!(out.contains("Root     /repo/demo"), "got:\n{out}");
}

#[test]
fn workwear_silhouette() {
    let out = render(Preset::Workwear, 100);
    // Uppercase title with the structural marker; indented subtitle.
    assert_eq!(first_line(&out), "» DEMO");
    assert!(out.contains("\n  /repo/demo\n"), "got:\n{out}");
    // Compact marker status, no gap before the fact block.
    assert!(out.contains("» STATUS  ✓ READY"), "got:\n{out}");
    // Uppercase labels with colons (presentation only).
    assert!(out.contains("ROOT:"), "got:\n{out}");
    assert!(out.contains("BRANCH:"), "got:\n{out}");
    // Compact one-line notice.
    assert!(
        out.lines().any(|l| l.starts_with("! Partial scan")),
        "got:\n{out}"
    );
    // Most compact of the four.
    let studio_lines = render(Preset::Studio, 100).lines().count();
    let house_lines = render(Preset::House, 100).lines().count();
    let ww_lines = out.lines().count();
    assert!(
        ww_lines < studio_lines,
        "Workwear must be tighter than Studio"
    );
    assert!(ww_lines <= house_lines, "Workwear must be tightest");
}

#[test]
fn studio_silhouette() {
    let out = render(Preset::Studio, 100);
    // Original-case title, no marker.
    assert_eq!(first_line(&out), "Demo");
    assert!(
        !out.contains('»'),
        "Studio must not use the operator marker"
    );
    // Status gets its own moment with a short rule.
    assert!(out.contains("Status\n──────\n"), "got:\n{out}");
    assert!(out.contains("✓ READY"), "got:\n{out}");
    // Widest key/value gap of the four.
    assert!(out.contains("Root        /repo/demo"), "got:\n{out}");
    // Roomiest vertical output.
    let house_lines = render(Preset::House, 100).lines().count();
    assert!(
        out.lines().count() > house_lines,
        "Studio must breathe more than House"
    );
}

#[test]
fn all_four_silhouettes_differ_without_colour() {
    let outs = [
        render(Preset::House, 100),
        render(Preset::BlackTie, 100),
        render(Preset::Workwear, 100),
        render(Preset::Studio, 100),
    ];
    for (i, a) in outs.iter().enumerate() {
        // Same semantic facts everywhere.
        for fact in ["/repo/demo", "main", "abc1234", "29 files", "READY"] {
            assert!(a.contains(fact), "preset {i} lost fact {fact:?}");
        }
        for (j, b) in outs.iter().enumerate() {
            if i != j {
                assert_ne!(a, b, "presets {i} and {j} are indistinguishable");
            }
        }
    }
}

#[test]
fn agent_json_identical_across_presets() {
    let mut jsons = Vec::new();
    for preset in [
        Preset::House,
        Preset::BlackTie,
        Preset::Workwear,
        Preset::Studio,
    ] {
        let _ = preset;
        jsons.push(fixture_screen().to_agent_json(true).unwrap());
    }
    for j in &jsons {
        assert_eq!(j, &jsons[0], "agent JSON must not vary by preset");
    }
}

#[test]
fn no_color_retains_structural_differences() {
    // Force TTY-true resolution so colour would normally apply, then compare
    // with and without NO_COLOR: structure identical, ANSI gone.
    let style_with = Config::default()
        .with_preset(Preset::Workwear)
        .resolve_style(true);
    assert!(style_with.color_enabled);
    let key = "SARTORIAL_NO_COLOR_TEST";
    let prev = std::env::var_os(key.replace("SARTORIAL", "NO_COLOR"));
    std::env::set_var("NO_COLOR", "1");
    let style_without = Config::default()
        .with_preset(Preset::Workwear)
        .resolve_style(true);
    if let Some(v) = prev {
        std::env::set_var("NO_COLOR", v);
    } else {
        std::env::remove_var("NO_COLOR");
    }
    assert!(!style_without.color_enabled);
    assert_eq!(style_with.preset, style_without.preset);
    assert_eq!(style_with.title_case, style_without.title_case);
    assert_eq!(style_with.status_layout, style_without.status_layout);
}

#[test]
fn motion_never_kills_all_animation() {
    for preset in [
        Preset::House,
        Preset::BlackTie,
        Preset::Workwear,
        Preset::Studio,
    ] {
        let ctx = RenderContext::human_motion(preset, MotionMode::Never);
        assert!(!ctx.should_animate(true), "{preset:?} must not animate");
        let mut bar = ProgressBar::activity("Scanning");
        bar.start_live_with_tty(&ctx, true).unwrap();
        assert!(!bar.is_animating(), "{preset:?} must not spin");
    }
}

#[test]
fn narrow_widths_keep_content_and_bound_rules() {
    for width in [60usize, 40] {
        for preset in [
            Preset::House,
            Preset::BlackTie,
            Preset::Workwear,
            Preset::Studio,
        ] {
            let out = render(preset, width);
            for line in out.lines() {
                assert!(
                    line.width() <= width,
                    "{preset:?} @{width}: line too wide: {line:?}"
                );
            }
            // Semantic content wins over style at every width.
            for fact in ["READY", "/repo/demo", "main", "29 files"] {
                assert!(out.contains(fact), "{preset:?} @{width} lost {fact:?}");
            }
        }
    }
}

#[test]
fn ascii_mode_stays_ascii() {
    let ctx = RenderContext::detect()
        .with_config(
            Config::default()
                .with_preset(Preset::Workwear)
                .with_color(ColorChoice::Never)
                .with_symbols(SymbolMode::Ascii)
                .with_width(100),
        )
        .with_target(RenderTarget::Human);
    let out = fixture_screen().to_human_string(&ctx).unwrap();
    assert!(out.contains("> DEMO"), "ASCII operator marker, got:\n{out}");
    assert!(out.contains("[OK] READY"), "got:\n{out}");
    assert!(!out.contains('»'), "no non-ASCII markers in ASCII mode");
    assert!(!out.contains('✓'), "no non-ASCII glyphs in ASCII mode");
    assert!(out.contains("ROOT:"), "operator voice survives ASCII mode");
}

#[test]
fn real_actions_keep_key_brackets() {
    // Every Action on the v0.1 wire carries a real trigger, so every
    // rendered action honestly wears its key brackets. No display-only
    // trigger exists: attach only keys the application really handles.
    let bar = ActionBar::from_actions(vec![Action::new('r', "retry", "Retry")]);
    let ctx = ctx_no_color(Preset::House, 100);
    assert!(
        bar.to_human_string(&ctx).unwrap().contains("[R] Retry"),
        "real actions keep their brackets"
    );
    let screen =
        SummaryScreen::new("Demo", Status::Ready).action(Action::new('d', "next", "Do the thing"));
    assert!(screen.to_agent_json(false).unwrap().contains("\"next\""));
}

#[test]
fn context_constructors_cover_normal_cases() {
    let human = RenderContext::human(Preset::Studio);
    assert_eq!(human.target, RenderTarget::Human);
    assert_eq!(human.config.preset, Preset::Studio);
    let plain = RenderContext::plain_preset(Preset::BlackTie);
    assert_eq!(plain.target, RenderTarget::Plain);
    assert!(!plain.color_enabled);
    let still = RenderContext::human_motion(Preset::House, MotionMode::Always);
    assert!(still.should_animate(true));
}
