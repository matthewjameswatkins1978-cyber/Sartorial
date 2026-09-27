use crate::components::action_bar::ActionBar;
use crate::components::section::Section;
use crate::components::title::Title;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::action::Action;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// A command, subcommand, or option description entry within HelpView.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelpEntry {
    pub name: String,
    pub description: String,
}

impl HelpEntry {
    pub fn new(name: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: desc.into(),
        }
    }
}

/// Sartorial house help presentation component embodying the Biscuit Logic CLI standard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelpView {
    pub program: String,
    pub description: Option<String>,
    pub usage: String,
    pub common_commands: Vec<HelpEntry>,
    pub output_flags: Vec<HelpEntry>,
    pub examples: Vec<String>,
    pub has_more_help: bool,
}

impl HelpView {
    pub fn new(program: impl Into<String>, usage: impl Into<String>) -> Self {
        Self {
            program: program.into(),
            description: None,
            usage: usage.into(),
            common_commands: Vec::new(),
            output_flags: Vec::new(),
            examples: Vec::new(),
            has_more_help: true,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn command(mut self, name: impl Into<String>, desc: impl Into<String>) -> Self {
        self.common_commands.push(HelpEntry::new(name, desc));
        self
    }

    pub fn output_flag(mut self, flag: impl Into<String>, desc: impl Into<String>) -> Self {
        self.output_flags.push(HelpEntry::new(flag, desc));
        self
    }

    pub fn example(mut self, ex: impl Into<String>) -> Self {
        self.examples.push(ex.into());
        self
    }

    pub fn with_more_help(mut self, has_more: bool) -> Self {
        self.has_more_help = has_more;
        self
    }
}

impl RenderHuman for HelpView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title = Title::new(&self.program);
        title.render_human(ctx, out)?;
        writeln!(out)?;

        if let Some(ref desc) = self.description {
            HumanRenderer::write_styled(out, ctx.style.muted_style(), desc, ctx.color_enabled)?;
            writeln!(out)?;
            writeln!(out)?;
        }

        // USAGE
        let sec_usage = Section::new("USAGE");
        sec_usage.render_human(ctx, out)?;
        write!(out, "  ")?;
        HumanRenderer::write_styled(out, ctx.style.value_style(), &self.usage, ctx.color_enabled)?;
        writeln!(out)?;
        writeln!(out)?;

        // COMMON
        if !self.common_commands.is_empty() {
            let sec_common = Section::new("COMMON");
            sec_common.render_human(ctx, out)?;

            let max_name = self
                .common_commands
                .iter()
                .map(|e| e.name.width())
                .max()
                .unwrap_or(10)
                .max(10);

            for cmd in &self.common_commands {
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.key_char_style(),
                    &format!("{:<width$}", cmd.name, width = max_name + 2),
                    ctx.color_enabled,
                )?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.muted_style(),
                    &cmd.description,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }
            writeln!(out)?;
        }

        // OUTPUT
        if !self.output_flags.is_empty() {
            let sec_output = Section::new("OUTPUT");
            sec_output.render_human(ctx, out)?;

            let max_flag = self
                .output_flags
                .iter()
                .map(|e| e.name.width())
                .max()
                .unwrap_or(10)
                .max(10);

            for opt in &self.output_flags {
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.value_style(),
                    &format!("{:<width$}", opt.name, width = max_flag + 2),
                    ctx.color_enabled,
                )?;
                HumanRenderer::write_styled(
                    out,
                    ctx.style.muted_style(),
                    &opt.description,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }
            writeln!(out)?;
        }

        // EXAMPLES
        if !self.examples.is_empty() {
            let sec_examples = Section::new("EXAMPLES");
            sec_examples.render_human(ctx, out)?;

            for ex in &self.examples {
                write!(out, "  ")?;
                HumanRenderer::write_styled(out, ctx.style.value_style(), ex, ctx.color_enabled)?;
                writeln!(out)?;
            }
            writeln!(out)?;
        }

        // FOOTER
        if self.has_more_help {
            let bar = ActionBar::new().with_action(Action::help().with_label("More help"));
            bar.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for HelpView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "{}", self.program)?;
        if let Some(ref desc) = self.description {
            writeln!(out, "\n{desc}")?;
        }
        writeln!(out, "\nUSAGE\n  {}", self.usage)?;

        if !self.common_commands.is_empty() {
            writeln!(out, "\nCOMMON")?;
            let max_name = self
                .common_commands
                .iter()
                .map(|e| e.name.width())
                .max()
                .unwrap_or(10)
                .max(10);
            for cmd in &self.common_commands {
                writeln!(
                    out,
                    "  {:<width$}{}",
                    cmd.name,
                    cmd.description,
                    width = max_name + 2
                )?;
            }
        }

        if !self.output_flags.is_empty() {
            writeln!(out, "\nOUTPUT")?;
            let max_flag = self
                .output_flags
                .iter()
                .map(|e| e.name.width())
                .max()
                .unwrap_or(10)
                .max(10);
            for opt in &self.output_flags {
                writeln!(
                    out,
                    "  {:<width$}{}",
                    opt.name,
                    opt.description,
                    width = max_flag + 2
                )?;
            }
        }

        if !self.examples.is_empty() {
            writeln!(out, "\nEXAMPLES")?;
            for ex in &self.examples {
                writeln!(out, "  {ex}")?;
            }
        }

        if self.has_more_help {
            writeln!(out)?;
            let bar = ActionBar::new().with_action(Action::help().with_label("More help"));
            bar.render_plain(ctx, out)?;
        }

        Ok(())
    }
}
