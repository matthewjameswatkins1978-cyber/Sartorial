use sartorial::*;
use unicode_width::UnicodeWidthStr;

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
    let narrow_output = kv.to_plain_string(&narrow_ctx).unwrap();

    // In narrow mode, label is stacked on its own line followed by indented value
    assert!(narrow_output.contains(
        "Application Data Directory:\n  C:\\Program Files\\Biscuit Logic\\App Data\\Config"
    ));

    // Normal context (80 columns)
    let normal_ctx = RenderContext::plain().with_width(80);
    assert!(!normal_ctx.is_narrow());
    let normal_output = kv.to_plain_string(&normal_ctx).unwrap();
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
    let output = detail.to_plain_string(&ctx).unwrap();

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

    assert_eq!(
        ready_badge.to_human_string(&unicode_ctx).unwrap(),
        "✓ READY"
    );
    assert_eq!(
        ready_badge.to_human_string(&ascii_ctx).unwrap(),
        "[OK] READY"
    );

    assert_eq!(
        failed_badge.to_human_string(&unicode_ctx).unwrap(),
        "× FAILED"
    );
    assert_eq!(
        failed_badge.to_human_string(&ascii_ctx).unwrap(),
        "[X] FAILED"
    );
}

#[test]
fn test_hostile_long_cells_narrow_table_enforces_width() {
    let mut table = TableModel::new(vec!["Component", "Path", "Status"]);
    let hostile_long_path = "D:\\Very\\Long\\Deeply\\Nested\\Directory\\Structure\\With\\Excessively\\Long\\Path\\Names\\That\\Exceed\\Terminal\\Width\\Entirely\\target\\release\\hostile_binary_name.exe";
    table.add_row(["Engine", hostile_long_path, "ready"]);
    table.add_row(["Parser", "src/parser/lexer.rs", "attention"]);

    let view = TableView::new(table);

    // Hostile narrow width: only 40 columns
    let ctx = RenderContext::plain().with_width(40);
    let plain_output = view.to_plain_string(&ctx).unwrap();

    // Verify EVERY line strictly conforms to width <= 40
    for line in plain_output.lines() {
        let display_width = UnicodeWidthStr::width(line);
        assert!(
            display_width <= 40,
            "Line exceeded 40 columns (width = {display_width}): '{line}'"
        );
    }

    // Verify truncation indicator was used on the hostile path
    assert!(plain_output.contains("…") || plain_output.contains("..."));
}

#[test]
fn test_unicode_cjk_width_handling() {
    let mut table = TableModel::new(vec!["Service", "Note", "Status"]);
    // CJK characters occupy 2 display cells each
    table.add_row(["東京クラスタ", "本番環境データセンター", "ready"]);
    table.add_row(["London", "UK Primary", "ready"]);

    let view = TableView::new(table);
    let ctx = RenderContext::plain().with_width(45);
    let plain_output = view.to_plain_string(&ctx).unwrap();

    for line in plain_output.lines() {
        let display_width = UnicodeWidthStr::width(line);
        assert!(
            display_width <= 45,
            "Line exceeded 45 columns (width = {display_width}): '{line}'"
        );
    }
}
