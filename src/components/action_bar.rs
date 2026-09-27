use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::Action;
use std::io::{self, Write};

/// Keyboard action footer teaching hotkeys and available interactions.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ActionBar {
    pub actions: Vec<Action>,
}

impl ActionBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_actions(actions: Vec<Action>) -> Self {
        Self { actions }
    }

    pub fn add(&mut self, action: Action) -> &mut Self {
        self.actions.push(action);
        self
    }

    pub fn with_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }
}

impl RenderHuman for ActionBar {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        if self.actions.is_empty() {
            return Ok(());
        }

        let gap = " ".repeat(ctx.style.action_gap(ctx.is_narrow()));
        let (open_bracket, close_bracket) = ctx.style.key_delimiters();

        for (idx, action) in self.actions.iter().enumerate() {
            let key_str = action.trigger.display_tag();

            HumanRenderer::write_styled(
                out,
                ctx.style.key_bracket_style(),
                open_bracket,
                ctx.color_enabled,
            )?;
            HumanRenderer::write_styled(
                out,
                ctx.style.key_char_style(),
                &key_str,
                ctx.color_enabled,
            )?;
            HumanRenderer::write_styled(
                out,
                ctx.style.key_bracket_style(),
                close_bracket,
                ctx.color_enabled,
            )?;
            write!(out, " ")?;
            HumanRenderer::write_styled(
                out,
                ctx.style.value_style(),
                &action.label,
                ctx.color_enabled,
            )?;

            if idx < self.actions.len() - 1 {
                write!(out, "{gap}")?;
            }
        }
        writeln!(out)
    }
}

impl RenderPlain for ActionBar {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let gap = " ".repeat(ctx.style.action_gap(ctx.is_narrow()));
        let (open_bracket, close_bracket) = ctx.style.key_delimiters();
        for (idx, action) in self.actions.iter().enumerate() {
            let key_str = action.trigger.display_tag();
            write!(
                out,
                "{open_bracket}{key_str}{close_bracket}{}{}",
                if action.label.is_empty() { "" } else { " " },
                action.label
            )?;
            if idx < self.actions.len() - 1 {
                write!(out, "{gap}")?;
            }
        }
        writeln!(out)
    }
}
