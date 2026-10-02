use sartorial::*;
use unicode_width::UnicodeWidthStr;

#[test]
fn test_table_absurdly_tiny_widths_strictly_bounded() {
    let mut table = TableModel::new(vec!["Tool", "Version", "Status", "ExtraLongCol"]);
    table.add_row(["Git", "2.51.0", "ready", "VeryLongSupplementalDetail"]);
    table.add_row([
        "Threadmoth",
        "1.10.0",
        "ready",
        "AnotherExcessivelyLongDetail",
    ]);

    let view = TableView::new(table);

    // Test a sweep of absurdly tiny terminal widths: 5, 8, 12, 15, 20
    for tiny_width in [5, 8, 12, 15, 20] {
        let ctx = RenderContext::plain().with_width(tiny_width);
        let plain = view.to_plain_string(&ctx).unwrap();

        for line in plain.lines() {
            let line_w = UnicodeWidthStr::width(line);
            assert!(
                line_w <= tiny_width,
                "Table line exceeded tiny width boundary {tiny_width} (actual {line_w}): '{line}'"
            );
        }
    }
}

#[test]
fn test_long_line_path_truncation() {
    let win_path = "C:\\Users\\Matmus\\.cargo\\bin\\tools\\external\\deeply\\nested\\vendor\\subproject\\target\\release\\threadmoth.exe";
    let truncated = sartorial::text::truncate_path(win_path, 40, "...");
    assert!(truncated.starts_with("C:\\Users\\"));
    assert!(truncated.ends_with("threadmoth.exe"));
    assert!(UnicodeWidthStr::width(truncated.as_str()) <= 40);
    assert!(truncated.contains("..."));

    let unix_path = "/var/log/application/deeply/nested/subsystem/audit/2026/09/27/trace.log";
    let truncated_unix = sartorial::text::truncate_path(unix_path, 35, "...");
    assert!(truncated_unix.starts_with("/var/log/"));
    assert!(truncated_unix.ends_with("trace.log"));
    assert!(UnicodeWidthStr::width(truncated_unix.as_str()) <= 35);
}

#[test]
fn test_choice_wrapped_lines_calculation() {
    let items = vec![
        ChoiceItem::new("item1", "Item One")
            .with_description("This is an extraordinarily long description designed to wrap across multiple rows in a narrow terminal."),
        ChoiceItem::new("item2", "Item Two")
            .with_description("Another long description for testing row wrapping arithmetic."),
    ];

    let choice = Choice::new("Select item:", items);

    // In an extra wide terminal (150 columns), 1 prompt line + 2 item lines = 3 lines total
    let wide_ctx = RenderContext::plain().with_width(150);
    assert_eq!(choice.calculate_rendered_lines(&wide_ctx), 3);

    // In a narrow terminal (30 columns), items wrap across multiple lines
    let narrow_ctx = RenderContext::plain().with_width(30);
    let narrow_lines = choice.calculate_rendered_lines(&narrow_ctx);
    assert!(
        narrow_lines > 3,
        "Expected wrapped lines to exceed 3 in narrow width, got {narrow_lines}"
    );
}
