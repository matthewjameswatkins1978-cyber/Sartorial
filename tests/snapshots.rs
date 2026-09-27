use sartorial::*;

#[test]
fn test_summary_screen_snapshot_plain() {
    let mut table = TableModel::new(vec!["Tool", "Version", "Status"])
        .with_title("Tools")
        .with_badge("9 / 11");
    table.add_row(["Git", "2.51.0", "ready"]);
    table.add_row(["ripgrep", "14.1.1", "ready"]);
    table.add_row(["fd", "", "missing"]);
    table.add_row(["Threadmoth", "1.10.0", "ready"]);

    let screen = SummaryScreen::new("SARTORIAL", Status::Ready)
        .with_table(table)
        .notice(Notice::info("2 optional tools can be installed."))
        .action(Action::new('i', "install", "Install"))
        .action(Action::find())
        .action(Action::help())
        .action(Action::quit());

    let ctx = RenderContext::plain().with_width(60);
    let plain_out = screen.to_plain_string(&ctx).unwrap();

    let expected = "\
SARTORIAL

Status                                            [OK] READY

Tools                                                 9 / 11
--------------------------------
Git           2.51.0     ready
ripgrep       14.1.1     ready
fd                       missing
Threadmoth    1.10.0     ready

2 optional tools can be installed.

[I] Install   [/] Find   [?] Help   [Q] Quit
";

    assert_eq!(
        plain_out.replace("\r\n", "\n"),
        expected.replace("\r\n", "\n")
    );
}

#[test]
fn test_error_view_snapshot_plain() {
    let err = ErrorModel::new("THREADMOTH NOT VISIBLE")
        .with_why("Threadmoth is installed, but this process cannot resolve it.")
        .with_evidence(Evidence::new("C:\\Users\\Matmus\\.cargo\\bin\\threadmoth.exe").at("Found:"))
        .with_action(Action::new('r', "recheck", "Recheck"))
        .with_action(Action::details());

    let ctx = RenderContext::plain();
    let plain_out = err.to_plain_string(&ctx).unwrap();

    let expected = "\
THREADMOTH NOT VISIBLE

Threadmoth is installed, but this process cannot resolve it.

Found:
C:\\Users\\Matmus\\.cargo\\bin\\threadmoth.exe

[R] Recheck   [D] Details
";

    assert_eq!(
        plain_out.replace("\r\n", "\n"),
        expected.replace("\r\n", "\n")
    );
}
