use crate::capabilities::Capabilities;
use crate::document::{Block, Document};
use crate::preset::TitleCase;
use crate::semantic::{NoticeLevel, ProgressMode};
use crate::style::ResolvedStyle;
use std::io::{self, Write};

/// First-class Markdown output for GitHub issues, PRs, release notes,
/// GitBook, agent reports, and handoffs.
///
/// Preserves semantic hierarchy (headings, tables, lists, callouts) rather
/// than reproducing terminal appearance. No ANSI, no width dependence.
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn render(
        doc: &Document,
        style: &ResolvedStyle,
        _caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let mut first = true;
        for block in &doc.blocks {
            if !first {
                writeln!(out)?;
            }
            first = false;
            Self::render_block(block, style, out)?;
        }
        Ok(())
    }

    pub fn render_to_string(
        doc: &Document,
        style: &ResolvedStyle,
        caps: &Capabilities,
    ) -> io::Result<String> {
        let mut buf = Vec::new();
        Self::render(doc, style, caps, &mut buf)?;
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    fn render_block(block: &Block, style: &ResolvedStyle, out: &mut dyn Write) -> io::Result<()> {
        match block {
            Block::Title { text, version } => {
                if let Some(ver) = version {
                    writeln!(out, "# {} `{ver}`", Self::heading_text(text, style))?;
                } else {
                    writeln!(out, "# {}", Self::heading_text(text, style))?;
                }
            }
            Block::Subtitle { text } => {
                writeln!(out, "*{text}*")?;
            }
            Block::StatusSection { label, status } => {
                writeln!(out, "## {}", Self::heading_text(label, style))?;
                writeln!(out)?;
                writeln!(
                    out,
                    "**{}:** {} {}",
                    label,
                    status.unicode_glyph(),
                    status.display_label()
                )?;
            }
            Block::BadgeSection { label, badge } => {
                if badge.is_empty() {
                    writeln!(out, "## {}", Self::heading_text(label, style))?;
                } else {
                    writeln!(out, "## {} — {badge}", Self::heading_text(label, style))?;
                }
            }
            Block::Summary { text } => {
                writeln!(out, "{text}")?;
            }
            Block::Facts { facts } => {
                writeln!(out, "| Name | Value |")?;
                writeln!(out, "| --- | --- |")?;
                for fact in facts {
                    let mut value = Self::escape_cell(&fact.value);
                    if let Some(unit) = &fact.unit {
                        value.push(' ');
                        value.push_str(&Self::escape_cell(unit));
                    }
                    writeln!(out, "| {} | {value} |", Self::escape_cell(&fact.name))?;
                }
            }
            Block::Evidence { items } => {
                writeln!(out, "### Evidence")?;
                writeln!(out)?;
                for ev in items {
                    if let Some(loc) = &ev.location {
                        writeln!(out, "- `{}` {}", Self::escape_inline(loc), ev.summary)?;
                    } else {
                        writeln!(out, "- {}", ev.summary)?;
                    }
                    if let Some(handle) = &ev.handle {
                        writeln!(out, "  - Ref: `{}`", Self::escape_inline(handle))?;
                    }
                }
            }
            Block::EvidenceDetail {
                section_title,
                item,
            } => {
                if let Some(title) = section_title {
                    writeln!(out, "### {}", Self::heading_text(title, style))?;
                    writeln!(out)?;
                }
                if let Some(loc) = &item.location {
                    writeln!(out, "`{}`", Self::escape_inline(loc))?;
                    writeln!(out)?;
                }
                writeln!(out, "{}", item.summary)?;
                if let Some(handle) = &item.handle {
                    writeln!(out)?;
                    writeln!(out, "Handle: `{}`", Self::escape_inline(handle))?;
                }
                if let Some(details) = &item.details {
                    writeln!(out)?;
                    writeln!(out, "```")?;
                    writeln!(out, "{details}")?;
                    writeln!(out, "```")?;
                }
            }
            Block::Notices { notices } => {
                for notice in notices {
                    Self::render_notice(notice, out)?;
                }
            }
            Block::Table { table } => {
                if let Some(title) = &table.title {
                    if let Some(badge) = &table.badge {
                        writeln!(out, "## {} — {badge}", Self::heading_text(title, style))?;
                    } else {
                        writeln!(out, "## {}", Self::heading_text(title, style))?;
                    }
                    writeln!(out)?;
                }
                if !table.headers.is_empty() {
                    writeln!(
                        out,
                        "| {} |",
                        table
                            .headers
                            .iter()
                            .map(|h| Self::escape_cell(h))
                            .collect::<Vec<_>>()
                            .join(" | ")
                    )?;
                    writeln!(
                        out,
                        "| {} |",
                        table
                            .headers
                            .iter()
                            .map(|_| "---")
                            .collect::<Vec<_>>()
                            .join(" | ")
                    )?;
                }
                for row in &table.rows {
                    writeln!(
                        out,
                        "| {} |",
                        row.cells
                            .iter()
                            .map(|c| Self::escape_cell(c))
                            .collect::<Vec<_>>()
                            .join(" | ")
                    )?;
                }
            }
            Block::Plan { plan } => {
                if let Some(desc) = &plan.description {
                    writeln!(out, "{desc}")?;
                    writeln!(out)?;
                }
                for change in &plan.changes {
                    if let Some(detail) = &change.detail {
                        writeln!(
                            out,
                            "- `{}` {} — {detail}",
                            change.kind.symbol(),
                            change.target
                        )?;
                    } else {
                        writeln!(out, "- `{}` {}", change.kind.symbol(), change.target)?;
                    }
                }
                if !plan.warnings.is_empty() {
                    writeln!(out)?;
                    for warn in &plan.warnings {
                        writeln!(out, "> [!WARNING]")?;
                        writeln!(out, "> {warn}")?;
                    }
                }
                if !plan.consequences.is_empty() {
                    writeln!(out)?;
                    writeln!(out, "### Consequences")?;
                    writeln!(out)?;
                    for c in &plan.consequences {
                        writeln!(out, "- {c}")?;
                    }
                }
                if let Some(rev) = plan.reversible {
                    writeln!(out)?;
                    writeln!(out, "**Reversible:** {}", if rev { "Yes" } else { "No" })?;
                }
                if !plan.actions.is_empty() {
                    writeln!(out)?;
                    Self::render_actions(&plan.actions, out)?;
                }
            }
            Block::Receipt { receipt } => {
                writeln!(out, "## {}", Self::heading_text(&receipt.title, style))?;
                writeln!(out)?;
                writeln!(
                    out,
                    "**Status:** {} {}",
                    receipt.status.unicode_glyph(),
                    receipt.status.display_label()
                )?;
                if !receipt.changes.is_empty() {
                    writeln!(out)?;
                    writeln!(out, "### Changes")?;
                    writeln!(out)?;
                    writeln!(out, "| Name | Value |")?;
                    writeln!(out, "| --- | --- |")?;
                    for f in &receipt.changes {
                        writeln!(
                            out,
                            "| {} | {} |",
                            Self::escape_cell(&f.name),
                            Self::escape_cell(&f.value)
                        )?;
                    }
                }
                if !receipt.unchanged.is_empty() {
                    writeln!(out)?;
                    writeln!(out, "### Unchanged")?;
                    writeln!(out)?;
                    for f in &receipt.unchanged {
                        writeln!(out, "- `{}`: {}", f.name, f.value)?;
                    }
                }
                if !receipt.warnings.is_empty() {
                    writeln!(out)?;
                    for warn in &receipt.warnings {
                        Self::render_notice(warn, out)?;
                    }
                }
                if let Some(guidance) = &receipt.guidance {
                    writeln!(out)?;
                    writeln!(out, "{guidance}")?;
                }
                if let Some(handle) = &receipt.evidence_handle {
                    writeln!(out)?;
                    writeln!(out, "Evidence reference: `{}`", Self::escape_inline(handle))?;
                }
                if !receipt.actions.is_empty() {
                    writeln!(out)?;
                    Self::render_actions(&receipt.actions, out)?;
                }
            }
            Block::Error { error } => {
                writeln!(out, "## {}", Self::heading_text(&error.what, style))?;
                writeln!(out)?;
                if let Some(why) = &error.why {
                    writeln!(out, "**Cause:** {why}")?;
                } else {
                    writeln!(
                        out,
                        "*Cause undetermined (no conclusive root cause established.)*"
                    )?;
                }
                if !error.evidence.is_empty() {
                    writeln!(out)?;
                    writeln!(out, "### Evidence")?;
                    writeln!(out)?;
                    for ev in &error.evidence {
                        if let Some(loc) = &ev.location {
                            writeln!(out, "- `{}` {}", Self::escape_inline(loc), ev.summary)?;
                        } else {
                            writeln!(out, "- {}", ev.summary)?;
                        }
                    }
                }
                if !error.next_actions.is_empty() {
                    writeln!(out)?;
                    writeln!(out, "### Next steps")?;
                    writeln!(out)?;
                    Self::render_actions(&error.next_actions, out)?;
                }
            }
            Block::Actions { actions } => {
                Self::render_actions(actions, out)?;
            }
            Block::Details { text } => {
                writeln!(out, "{text}")?;
            }
            Block::Choices { items } => {
                for item in items {
                    if item.disabled {
                        writeln!(out, "- [ ] ~~{}~~ (unavailable)", item.label)?;
                    } else {
                        writeln!(out, "- [ ] {}", item.label)?;
                    }
                }
            }
            Block::List { ordered, items } => {
                for (idx, item) in items.iter().enumerate() {
                    if *ordered {
                        writeln!(out, "{}. {item}", idx + 1)?;
                    } else {
                        writeln!(out, "- {item}")?;
                    }
                }
            }
            Block::ProgressSnapshot { state } => {
                let mut line = state.task.clone();
                match state.mode {
                    ProgressMode::Activity => {
                        if let Some(e) = state.elapsed_secs {
                            line.push_str(&format!(" ({e}s elapsed)"));
                        }
                    }
                    ProgressMode::Count | ProgressMode::Rate => {
                        if let (Some(c), Some(t)) = (state.current, state.total) {
                            line.push_str(&format!(" ({c}/{t}"));
                            if let Some(p) = state.derived_percent() {
                                line.push_str(&format!(", {p}%"));
                            }
                            line.push(')');
                        }
                        if let Some(r) = &state.rate {
                            line.push_str(&format!(" {r}"));
                        }
                    }
                    ProgressMode::Percent => {
                        if let Some(p) = state.derived_percent() {
                            line.push_str(&format!(" ({p}%)"));
                        }
                    }
                    ProgressMode::Countdown => {
                        if let Some(s) = state.countdown_secs {
                            line.push_str(&format!(" ({s}s remaining)"));
                        }
                    }
                }
                writeln!(out, "`{line}`")?;
            }
        }
        Ok(())
    }

    fn render_notice(notice: &crate::semantic::Notice, out: &mut dyn Write) -> io::Result<()> {
        let tag = match notice.level {
            NoticeLevel::Info => "[!NOTE]",
            NoticeLevel::Tip => "[!TIP]",
            NoticeLevel::Warning => "[!WARNING]",
            NoticeLevel::Error => "[!CAUTION]",
        };
        writeln!(out, "> {tag}")?;
        writeln!(out, "> {}", notice.message)?;
        if let Some(detail) = &notice.detail {
            writeln!(out, ">")?;
            writeln!(out, "> {detail}")?;
        }
        Ok(())
    }

    fn render_actions(actions: &[crate::semantic::Action], out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "### Actions")?;
        writeln!(out)?;
        for action in actions {
            writeln!(
                out,
                "- **{}** — {}",
                action.trigger.display_tag(),
                action.label
            )?;
        }
        Ok(())
    }

    /// Headings keep preset casing but drop terminal structural markers
    /// (`» `), which read as noise in Markdown.
    fn heading_text(text: &str, style: &ResolvedStyle) -> String {
        match style.title_case {
            TitleCase::Upper => text.to_uppercase(),
            TitleCase::Preserve => text.to_string(),
        }
    }

    fn escape_cell(text: &str) -> String {
        text.replace('|', "\\|").replace('\n', "<br>")
    }

    fn escape_inline(text: &str) -> String {
        text.replace('`', "'")
    }
}
