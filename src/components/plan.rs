use crate::components::action_bar::ActionBar;
use crate::components::title::Title;
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::plan::Plan;
use std::io::{self, Write};

impl RenderHuman for Plan {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_human(ctx, out)?;
        writeln!(out)?;

        if let Some(ref desc) = self.description {
            HumanRenderer::write_styled(
                out,
                HumanRenderer::section_style(),
                desc,
                ctx.color_enabled,
            )?;
            writeln!(out)?;
            writeln!(out)?;
        }

        // Changes list
        for change in &self.changes {
            let symbol = change.kind.symbol();
            if ctx.color_enabled {
                let sym_style = anstyle::Style::new()
                    .fg_color(Some(change.kind.color().into()))
                    .effects(anstyle::Effects::BOLD);
                HumanRenderer::write_styled(out, sym_style, symbol, true)?;
            } else {
                write!(out, "{symbol}")?;
            }
            write!(out, " ")?;
            HumanRenderer::write_styled(
                out,
                HumanRenderer::value_style(),
                &change.target,
                ctx.color_enabled,
            )?;
            writeln!(out)?;

            if let Some(ref detail) = change.detail {
                write!(out, "  ")?;
                HumanRenderer::write_styled(
                    out,
                    HumanRenderer::muted_style(),
                    detail,
                    ctx.color_enabled,
                )?;
                writeln!(out)?;
            }
        }

        if !self.changes.is_empty() && (!self.consequences.is_empty() || !self.warnings.is_empty())
        {
            writeln!(out)?;
        }

        // Warnings
        for warn in &self.warnings {
            let prefix = if ctx.symbols == crate::config::SymbolMode::Ascii {
                "[!] "
            } else {
                "! "
            };
            if ctx.color_enabled {
                let warn_style = anstyle::Style::new()
                    .fg_color(Some(anstyle::AnsiColor::Yellow.into()))
                    .effects(anstyle::Effects::BOLD);
                HumanRenderer::write_styled(out, warn_style, prefix, true)?;
                HumanRenderer::write_styled(out, warn_style, warn, true)?;
            } else {
                write!(out, "{prefix}{warn}")?;
            }
            writeln!(out)?;
        }

        // Consequences
        for c in &self.consequences {
            HumanRenderer::write_styled(out, HumanRenderer::muted_style(), c, ctx.color_enabled)?;
            writeln!(out)?;
        }

        if let Some(rev) = self.reversible {
            let rev_text = if rev {
                "Reversible: Yes (can be rolled back)"
            } else {
                "Reversible: No (irreversible operation)"
            };
            HumanRenderer::write_styled(
                out,
                HumanRenderer::muted_style(),
                rev_text,
                ctx.color_enabled,
            )?;
            writeln!(out)?;
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let bar = ActionBar::from_actions(self.actions.clone());
            bar.render_human(ctx, out)?;
        }

        Ok(())
    }
}

impl RenderPlain for Plan {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        let title_comp = Title::new(&self.title);
        title_comp.render_plain(ctx, out)?;
        writeln!(out)?;

        if let Some(ref desc) = self.description {
            writeln!(out, "{desc}\n")?;
        }

        for change in &self.changes {
            writeln!(out, "{} {}", change.kind.symbol(), change.target)?;
            if let Some(ref detail) = change.detail {
                writeln!(out, "  {detail}")?;
            }
        }

        if !self.changes.is_empty() && (!self.consequences.is_empty() || !self.warnings.is_empty())
        {
            writeln!(out)?;
        }

        for warn in &self.warnings {
            writeln!(out, "! {warn}")?;
        }

        for c in &self.consequences {
            writeln!(out, "{c}")?;
        }

        if let Some(rev) = self.reversible {
            let rev_text = if rev {
                "Reversible: Yes"
            } else {
                "Reversible: No"
            };
            writeln!(out, "{rev_text}")?;
        }

        if !self.actions.is_empty() {
            writeln!(out)?;
            let bar = ActionBar::from_actions(self.actions.clone());
            bar.render_plain(ctx, out)?;
        }

        Ok(())
    }
}
