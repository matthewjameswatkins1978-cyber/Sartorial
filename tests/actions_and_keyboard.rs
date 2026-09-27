use sartorial::semantic::action::KeyTrigger;
use sartorial::*;

#[test]
fn test_standard_biscuit_logic_actions() {
    let open = Action::open();
    assert_eq!(open.id, "open");
    assert_eq!(open.label, "Open");
    assert_eq!(open.trigger, KeyTrigger::Enter);
    assert!(open.is_default);

    let back = Action::back();
    assert_eq!(back.id, "back");
    assert_eq!(back.label, "Back");
    assert_eq!(back.trigger, KeyTrigger::Esc);

    let find = Action::find();
    assert_eq!(find.id, "find");
    assert_eq!(find.label, "Find");
    assert_eq!(find.trigger, KeyTrigger::Char('/'));

    let help = Action::help();
    assert_eq!(help.id, "help");
    assert_eq!(help.label, "Help");
    assert_eq!(help.trigger, KeyTrigger::Char('?'));

    let details = Action::details();
    assert_eq!(details.id, "details");
    assert_eq!(details.label, "Details");
    assert!(details.trigger.matches_char('d'));
    assert!(details.trigger.matches_char('D'));

    let retry = Action::retry();
    assert_eq!(retry.id, "retry");
    assert_eq!(retry.label, "Retry");
    assert!(retry.trigger.matches_char('r'));

    let quit = Action::quit();
    assert_eq!(quit.id, "quit");
    assert_eq!(quit.label, "Quit");
    assert!(quit.trigger.matches_char('q'));
}

#[test]
fn test_action_bar_rendering() {
    let bar = ActionBar::new()
        .with_action(Action::new('i', "install", "Install"))
        .with_action(Action::find())
        .with_action(Action::help())
        .with_action(Action::quit());

    let ctx = RenderContext::plain();
    let rendered = bar.to_plain_string(&ctx);

    assert!(rendered.contains("[I] Install"));
    assert!(rendered.contains("[/] Find"));
    assert!(rendered.contains("[?] Help"));
    assert!(rendered.contains("[Q] Quit"));
}
