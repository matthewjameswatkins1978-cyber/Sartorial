use sartorial::*;

#[test]
fn test_semantic_exit_code_contract() {
    assert_eq!(ExitCode::Success.as_i32(), 0);
    assert_eq!(ExitCode::UsageError.as_i32(), 2);
    assert_eq!(ExitCode::Unavailable.as_i32(), 3);
    assert_eq!(ExitCode::Failed.as_i32(), 4);
    assert_eq!(ExitCode::Cancelled.as_i32(), 130);

    let code: i32 = ExitCode::Failed.into();
    assert_eq!(code, 4);
}

#[test]
fn test_pager_policy_safe_guards() {
    let pager = PagerMode::Auto;
    // Machine targets (agent JSON and plain) MUST NEVER page
    assert!(!pager.should_page(true, true, false, 100, 24));
    assert!(!pager.should_page(true, false, true, 100, 24));
    // Non-interactive redirected streams must never page
    assert!(!pager.should_page(false, false, false, 100, 24));
    // Attended terminal with fewer lines than height should not page
    assert!(!pager.should_page(true, false, false, 15, 24));
    // Attended terminal with output exceeding screen height should page
    assert!(pager.should_page(true, false, false, 50, 24));
}

#[test]
fn test_output_channel_progress_suppressed_in_agent_mode() {
    // In agent mode, print_progress must silently no-op to protect stdout/stderr purity
    let ctx = RenderContext::agent();
    let pb = ProgressBar::activity("Silent background task");
    let res = SartorialOutput::print_progress(&pb, &ctx);
    assert!(res.is_ok());
}
