use sartorial::*;

#[test]
fn test_preset_defaults_and_resolution() {
    let house_cfg = Config::default();
    assert_eq!(house_cfg.preset, Preset::House);
    assert_eq!(house_cfg.accent, anstyle::AnsiColor::Cyan);
    assert_eq!(house_cfg.density, Density::Standard);

    let bt_cfg = Config::new().with_preset(Preset::BlackTie);
    assert_eq!(bt_cfg.preset, Preset::BlackTie);
    assert_eq!(bt_cfg.accent, anstyle::AnsiColor::BrightWhite);

    let ww_cfg = Config::new().with_preset(Preset::Workwear);
    assert_eq!(ww_cfg.preset, Preset::Workwear);
    assert_eq!(ww_cfg.accent, anstyle::AnsiColor::Yellow);
    assert_eq!(ww_cfg.density, Density::Compact);

    let st_cfg = Config::new().with_preset(Preset::Studio);
    assert_eq!(st_cfg.preset, Preset::Studio);
    assert_eq!(st_cfg.accent, anstyle::AnsiColor::Magenta);
    assert_eq!(st_cfg.density, Density::Roomy);
}

#[test]
fn test_preset_override_authority() {
    // Explicit override takes precedence over preset defaults
    let cfg = Config::new()
        .with_preset(Preset::Workwear)
        .with_accent(anstyle::AnsiColor::Green)
        .with_density(Density::Roomy);

    assert_eq!(cfg.preset, Preset::Workwear);
    assert_eq!(cfg.accent, anstyle::AnsiColor::Green);
    assert_eq!(cfg.density, Density::Roomy);

    let resolved = cfg.resolve_style(true);
    assert_eq!(resolved.preset, Preset::Workwear);
    assert_eq!(resolved.accent, anstyle::AnsiColor::Green);
    assert_eq!(resolved.density, Density::Roomy);
    assert_eq!(resolved.action_spacing, 2); // Workwear spacing
}

#[test]
fn test_same_semantic_content_across_all_four_presets() {
    let outcome = Outcome::new(Status::Ready, "Verification Passed")
        .with_summary("All checks passed")
        .fact("Target", "Release")
        .with_action(Action::open())
        .with_action(Action::quit());

    let presets = [
        Preset::House,
        Preset::BlackTie,
        Preset::Workwear,
        Preset::Studio,
    ];

    for p in presets {
        let ctx = RenderContext::plain().with_config(Config::new().with_preset(p));
        let plain = outcome.to_plain_string(&ctx).unwrap();

        // Semantic invariant: essential data is identical across all presets.
        // Title casing is presentation grammar: House/Workwear uppercase,
        // Black Tie/Studio preserve the application-supplied case.
        let title = match p {
            Preset::House | Preset::Workwear => "VERIFICATION PASSED",
            Preset::BlackTie | Preset::Studio => "Verification Passed",
        };
        assert!(plain.contains(title));
        assert!(plain.contains("All checks passed"));
        // Fact labels follow the same grammar: operator voice uppercases.
        let fact = match p {
            Preset::Workwear => "TARGET:",
            _ => "Target",
        };
        assert!(plain.contains(fact));
        assert!(plain.contains("Release"));
        assert!(plain.contains("Open"));
        assert!(plain.contains("Quit"));

        if p == Preset::BlackTie {
            assert!(plain.contains("(Enter) Open"));
            assert!(plain.contains("(Q) Quit"));
        } else {
            assert!(plain.contains("[Enter] Open"));
            assert!(plain.contains("[Q] Quit"));
        }
    }
}
