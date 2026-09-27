//! Plain-target ASCII safety: pipe-safe output must never leak Unicode
//! structural or status glyphs, however the context was resolved.

use sartorial::*;

fn rich_screen() -> SummaryScreen {
    let mut table = TableModel::new(vec!["Tool", "Version", "Status"]).with_title("Tools");
    table.add_row(["Git", "2.51.0", "ready"]);
    table.add_row(["fd", "", "missing"]);
    SummaryScreen::new("Demo", Status::Ready)
        .with_subtitle("/repo/demo")
        .fact("Root", "/repo/demo")
        .fact("Files", "29 files")
        .with_table(table)
        .notice(Notice::warning("Partial scan: 1 warning"))
        .action(Action::new('r', "retry", "Retry"))
}

#[test]
fn workwear_plain_preset_is_ascii_safe() {
    let ctx = RenderContext::plain_preset(Preset::Workwear).with_width(100);
    let out = rich_screen().to_plain_string(&ctx).unwrap();
    assert!(
        out.contains("> "),
        "operator marker must survive in ASCII form, got:\n{out}"
    );
    assert!(
        out.contains("[OK] READY"),
        "ASCII status badge expected, got:\n{out}"
    );
    assert!(
        out.is_ascii(),
        "plain Workwear must be pure ASCII, got:\n{out}"
    );
    for glyph in ['»', '✓', '×', '●', '○', '–', '─', '↑', '↓', '←', '→'] {
        assert!(
            !out.contains(glyph),
            "plain output leaked {glyph:?}:\n{out}"
        );
    }
}

#[test]
fn with_target_plain_re_resolves_unicode_markers() {
    // Resolve Unicode grammar first (as a TTY session would), then switch
    // to Plain: no stale Unicode structural marker may survive the switch.
    let ctx = RenderContext::detect()
        .with_config(
            Config::default()
                .with_preset(Preset::Workwear)
                .with_color(ColorChoice::Never)
                .with_symbols(SymbolMode::Unicode)
                .with_width(100),
        )
        .with_target(RenderTarget::Plain);
    assert_eq!(ctx.style.title_marker, "> ");
    assert_eq!(ctx.style.section_marker, "> ");
    assert_eq!(ctx.style.rule_char, '-');
    let out = rich_screen().to_plain_string(&ctx).unwrap();
    assert!(
        out.contains("> "),
        "ASCII operator marker expected after target switch, got:\n{out}"
    );
    assert!(
        out.is_ascii(),
        "plain output after target switch must be pure ASCII, got:\n{out}"
    );
    assert!(!out.contains('»'), "stale Unicode marker leaked:\n{out}");
    assert!(!out.contains('✓'), "stale Unicode glyph leaked:\n{out}");
}
