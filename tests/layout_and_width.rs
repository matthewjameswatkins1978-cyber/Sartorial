use sartorial::*;

#[test]
fn test_narrow_terminal_stacking() {
    let mut kv = KeyValueList::new();
    kv.add(
        "Application Data Directory",
        "C:\\Program Files\\Biscuit Logic\\App Data\\Config",
    );
    kv.add("Environment State", "Production");

    // Narrow context (< 60 columns)
    let narrow_ctx = RenderContext::plain().with_width(45);
    assert!(narrow_ctx.is_narrow());
    let narrow_output = kv.to_plain_string(&narrow_ctx);

    // In narrow mode, label is stacked on its own line followed by indented value
    assert!(narrow_output.contains(
        "Application Data Directory:\n  C:\\Program Files\\Biscuit Logic\\App Data\\Config"
    ));

    // Normal context (80 columns)
    let normal_ctx = RenderContext::plain().with_width(80);
    assert!(!normal_ctx.is_narrow());
    let normal_output = kv.to_plain_string(&normal_ctx);
    // In normal mode, aligned on same line
    assert!(normal_output.contains(
        "Application Data Directory  C:\\Program Files\\Biscuit Logic\\App Data\\Config"
    ));
}

#[test]
fn test_paths_with_spaces() {
    let evidence = Evidence::new("Binary resolved")
        .at("C:\\Program Files\\Common Files\\System Utilities\\runner.exe")
        .with_handle("exec-ref-91");

    let detail = DetailView::new("Path Resolution", evidence);
    let ctx = RenderContext::plain();
    let output = detail.to_plain_string(&ctx);

    assert!(output.contains("C:\\Program Files\\Common Files\\System Utilities\\runner.exe"));
    assert!(output.contains("Binary resolved"));
    assert!(output.contains("exec-ref-91"));
}

#[test]
fn test_unicode_and_ascii_symbol_modes() {
    let ready_badge = StatusBadge::new(Status::Ready);
    let failed_badge = StatusBadge::new(Status::Failed);

    let unicode_ctx = RenderContext::detect().with_config(
        Config::new()
            .with_symbols(SymbolMode::Unicode)
            .with_color(ColorChoice::Never),
    );
    let ascii_ctx = RenderContext::detect().with_config(
        Config::new()
            .with_symbols(SymbolMode::Ascii)
            .with_color(ColorChoice::Never),
    );

    assert_eq!(ready_badge.to_human_string(&unicode_ctx), "✓ READY");
    assert_eq!(ready_badge.to_human_string(&ascii_ctx), "[OK] READY");

    assert_eq!(failed_badge.to_human_string(&unicode_ctx), "× FAILED");
    assert_eq!(failed_badge.to_human_string(&ascii_ctx), "[X] FAILED");
}
