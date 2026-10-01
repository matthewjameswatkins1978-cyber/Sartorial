use crate::capabilities::Capabilities;
use crate::document::{Block, Document};
use crate::preset::{SectionRule, StatusLayout};
use crate::semantic::{ColumnAlignment, ProgressMode};
use crate::style::ResolvedStyle;
use crate::text::truncate_with_ellipsis;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Pipe-safe and log-safe deterministic output: no ANSI, ASCII glyphs,
/// safe redirection, meaningful without colour, no loss of semantic truth.
pub struct PlainRenderer;

impl PlainRenderer {
    pub fn render(
        doc: &Document,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let narrow = caps.is_narrow();
        let mut first = true;
        for block in &doc.blocks {
            if !first {
                for _ in 0..style.component_gap(narrow) {
                    writeln!(out)?;
                }
            }
            first = false;
            Self::render_block(block, style, caps, out)?;
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

    fn render_block(
        block: &Block,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        match block {
            Block::Title { text, version } => {
                let title_text = style.format_title(text);
                if let Some(ver) = version {
                    writeln!(out, "{title_text} {ver}")
                } else {
                    writeln!(out, "{title_text}")
                }
            }
            Block::Subtitle { text } => {
                let indent = " ".repeat(style.title_marker.width());
                writeln!(out, "{indent}{text}")
            }
            Block::StatusSection { label, status } => {
                let title_text = style.format_section_header(label);
                write!(out, "{title_text}")?;
                match style.status_layout {
                    StatusLayout::Inline => {
                        write!(out, "{}", " ".repeat(style.status_gap))?;
                        Self::write_status(out, *status)?;
                        writeln!(out)?;
                    }
                    StatusLayout::Stacked => {
                        writeln!(out)?;
                        Self::write_heading_rule(out, style, title_text.width())?;
                        Self::write_status(out, *status)?;
                        writeln!(out)?;
                    }
                }
                Ok(())
            }
            Block::BadgeSection { label, badge } => {
                Self::render_badge_section(label, badge, style, out)
            }
            Block::Summary { text } => writeln!(out, "{text}"),
            Block::Facts { facts } => {
                let labels: Vec<String> = facts
                    .iter()
                    .map(|f| style.format_fact_label(&f.name))
                    .collect();
                let max_label_len = labels.iter().map(|l| l.width()).max().unwrap_or(0);
                let narrow = caps.is_narrow();
                let pad_spacing = style.fact_pad(narrow);
                let pad_label = (max_label_len + pad_spacing).min(30);
                for (fact, label) in facts.iter().zip(labels.iter()) {
                    if narrow {
                        if label.ends_with(':') {
                            writeln!(out, "{label}\n  {}", fact.value)?;
                        } else {
                            writeln!(out, "{label}:\n  {}", fact.value)?;
                        }
                    } else {
                        let label_width = label.width();
                        let pad = if pad_label > label_width {
                            pad_label - label_width
                        } else {
                            pad_spacing
                        };
                        let unit_suffix = fact
                            .unit
                            .as_ref()
                            .map(|u| format!(" {u}"))
                            .unwrap_or_default();
                        writeln!(
                            out,
                            "{label}{}{}{}",
                            " ".repeat(pad),
                            fact.value,
                            unit_suffix
                        )?;
                    }
                }
                Ok(())
            }
            Block::Evidence { items } => {
                for ev in items {
                    if let Some(loc) = &ev.location {
                        writeln!(out, "{loc}")?;
                    }
                    writeln!(out, "{}", ev.summary)?;
                }
                Ok(())
            }
            Block::EvidenceDetail {
                section_title,
                item,
            } => {
                if let Some(title) = section_title {
                    let heading = style.format_section_header(title);
                    writeln!(out, "{heading}")?;
                    Self::write_heading_rule(out, style, heading.width())?;
                }
                if let Some(loc) = &item.location {
                    writeln!(out, "{loc}")?;
                }
                writeln!(out, "{}", item.summary)?;
                if let Some(handle) = &item.handle {
                    writeln!(out, "Handle: {handle}")?;
                }
                if let Some(details) = &item.details {
                    writeln!(out)?;
                    for line in details.lines() {
                        writeln!(out, "{line}")?;
                    }
                }
                Ok(())
            }
            Block::Notices { notices } => {
                for notice in notices {
                    if notice.level == crate::semantic::NoticeLevel::Info {
                        write!(out, "{}", notice.message)?;
                    } else {
                        write!(out, "{} {}", notice.level.ascii_glyph(), notice.message)?;
                    }
                    if let Some(detail) = &notice.detail {
                        write!(out, " {detail}")?;
                    }
                    writeln!(out)?;
                }
                Ok(())
            }
            Block::Table { table } => {
                if let Some(title) = &table.title {
                    let badge = table.badge.clone().unwrap_or_default();
                    Self::render_badge_section(title, &badge, style, out)?;
                }
                Self::render_table_rows(table, style, caps, out)
            }
            Block::Plan { plan } => {
                if let Some(desc) = &plan.description {
                    writeln!(out, "{desc}\n")?;
                }
                for change in &plan.changes {
                    writeln!(out, "{} {}", change.kind.symbol(), change.target)?;
                    if let Some(detail) = &change.detail {
                        writeln!(out, "  {detail}")?;
                    }
                }
                if !plan.changes.is_empty()
                    && (!plan.consequences.is_empty() || !plan.warnings.is_empty())
                {
                    writeln!(out)?;
                }
                for warn in &plan.warnings {
                    writeln!(out, "! {warn}")?;
                }
                for c in &plan.consequences {
                    writeln!(out, "{c}")?;
                }
                if let Some(rev) = plan.reversible {
                    writeln!(
                        out,
                        "{}",
                        if rev {
                            "Reversible: Yes"
                        } else {
                            "Reversible: No"
                        }
                    )?;
                }
                if !plan.actions.is_empty() {
                    writeln!(out)?;
                    Self::render_actions(&plan.actions, style, caps, out)?;
                }
                Ok(())
            }
            Block::Receipt { receipt } => {
                Self::write_status(out, receipt.status)?;
                writeln!(out, " {}", receipt.title)?;
                writeln!(out)?;
                if !receipt.changes.is_empty() {
                    Self::render_block(
                        &Block::Facts {
                            facts: receipt.changes.clone(),
                        },
                        style,
                        caps,
                        out,
                    )?;
                }
                if !receipt.unchanged.is_empty() {
                    writeln!(out, "\nUnchanged:")?;
                    Self::render_block(
                        &Block::Facts {
                            facts: receipt.unchanged.clone(),
                        },
                        style,
                        caps,
                        out,
                    )?;
                }
                for warn in &receipt.warnings {
                    writeln!(out)?;
                    Self::render_block(
                        &Block::Notices {
                            notices: vec![warn.clone()],
                        },
                        style,
                        caps,
                        out,
                    )?;
                }
                if let Some(guidance) = &receipt.guidance {
                    writeln!(out, "\n{guidance}")?;
                }
                if let Some(handle) = &receipt.evidence_handle {
                    writeln!(out, "\nEvidence reference: {handle}")?;
                }
                if !receipt.actions.is_empty() {
                    writeln!(out)?;
                    Self::render_actions(&receipt.actions, style, caps, out)?;
                }
                Ok(())
            }
            Block::Error { error } => {
                let what = match style.title_case {
                    crate::preset::TitleCase::Upper => error.what.to_uppercase(),
                    crate::preset::TitleCase::Preserve => error.what.clone(),
                };
                writeln!(out, "{what}\n")?;
                if let Some(why) = &error.why {
                    writeln!(out, "{why}")?;
                } else {
                    writeln!(
                        out,
                        "Cause undetermined (no conclusive root cause established)."
                    )?;
                }
                if !error.evidence.is_empty() {
                    writeln!(out)?;
                    for ev in &error.evidence {
                        if let Some(loc) = &ev.location {
                            writeln!(out, "{loc}")?;
                        }
                        writeln!(out, "{}", ev.summary)?;
                        if let Some(handle) = &ev.handle {
                            writeln!(out, "Ref: {handle}")?;
                        }
                    }
                }
                if !error.next_actions.is_empty() {
                    writeln!(out)?;
                    Self::render_actions(&error.next_actions, style, caps, out)?;
                }
                Ok(())
            }
            Block::Actions { actions } => Self::render_actions(actions, style, caps, out),
            Block::Details { text } => writeln!(out, "{text}"),
            Block::Choices { items } => {
                for item in items {
                    if item.disabled {
                        writeln!(out, "  - {} (unavailable)", item.label)?;
                    } else {
                        writeln!(out, "  - {}", item.label)?;
                    }
                }
                Ok(())
            }
            Block::List { ordered, items } => {
                for (idx, item) in items.iter().enumerate() {
                    if *ordered {
                        writeln!(out, "{:2}. {}", idx + 1, item)?;
                    } else {
                        writeln!(out, "- {}", item)?;
                    }
                }
                Ok(())
            }
            Block::ProgressSnapshot { state } => {
                let mut line = state.task.clone();
                if let Some(sub) = &state.subtask {
                    line.push_str(&format!(" ({sub})"));
                }
                match state.mode {
                    ProgressMode::Activity => {
                        if let Some(e) = state.elapsed_secs {
                            line.push_str(&format!(" [{e}s]"));
                        }
                    }
                    ProgressMode::Count | ProgressMode::Rate => {
                        if let (Some(c), Some(t)) = (state.current, state.total) {
                            line.push_str(&format!(" {c}/{t}"));
                            if let Some(p) = state.derived_percent() {
                                line.push_str(&format!(" ({p}%)"));
                            }
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
                            line.push_str(&format!(" ({s}s)"));
                        }
                    }
                }
                writeln!(out, "{line}")
            }
        }
    }

    fn render_title(
        text: &str,
        version: Option<&str>,
        subtitle: Option<&str>,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        // Plain keeps the layout grammar (casing, markers) with ASCII symbols.
        let title_text = style.format_title(text);
        if let Some(ver) = version {
            writeln!(out, "{title_text} {ver}")?;
        } else {
            writeln!(out, "{title_text}")?;
        }
        if let Some(sub) = subtitle {
            let indent = " ".repeat(style.title_marker.width());
            writeln!(out, "{indent}{sub}")?;
        }
        if style.title_block_rule {
            Self::write_rule(out, caps, style.title_rule_len(caps.width))?;
        }
        Ok(())
    }

    /// Title with subtitle and grammar rule.
    pub fn render_title_block(
        text: &str,
        version: Option<&str>,
        subtitle: Option<&str>,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        Self::render_title(text, version, subtitle, style, caps, out)
    }

    fn render_badge_section(
        label: &str,
        badge: &str,
        style: &ResolvedStyle,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let title_text = style.format_section_header(label);
        if badge.is_empty() {
            writeln!(out, "{title_text}")?;
            Self::write_heading_rule(out, style, title_text.width())?;
            return Ok(());
        }
        match style.status_layout {
            StatusLayout::Inline => {
                writeln!(out, "{title_text}{}{badge}", " ".repeat(style.status_gap))?;
            }
            StatusLayout::Stacked => {
                writeln!(out, "{title_text}\n  {badge}")?;
            }
        }
        Ok(())
    }

    fn render_table_rows(
        table: &crate::semantic::TableModel,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let narrow = caps.is_narrow();
        let base_col_gap = style.table_col_gap(narrow);
        let (widths, col_gap) = super::terminal::column_widths_for(table, caps.width, base_col_gap);
        let active_cols: Vec<usize> = (0..widths.len()).filter(|&i| widths[i] > 0).collect();
        if active_cols.is_empty() {
            return Ok(());
        }
        let total_content_width =
            widths.iter().sum::<usize>() + (active_cols.len().saturating_sub(1) * col_gap);
        let rule_len = total_content_width.min(caps.width);

        if style.table_headers && !table.headers.is_empty() {
            for (pos, &i) in active_cols.iter().enumerate() {
                let cell = table.headers.get(i).map(|s| s.as_str()).unwrap_or("");
                let col_width = widths.get(i).copied().unwrap_or(cell.width());
                let cell_truncated = truncate_with_ellipsis(cell, col_width, "...");
                let is_last = pos == active_cols.len() - 1;
                let formatted = if is_last {
                    cell_truncated
                } else {
                    super::terminal::pad_cell_for(&cell_truncated, col_width, ColumnAlignment::Left)
                };
                write!(out, "{formatted}")?;
                if !is_last {
                    write!(out, "{}", " ".repeat(col_gap))?;
                }
            }
            writeln!(out)?;
        }

        // Plain always carries the rule (v0.2 PlainRenderer::write_rule).
        Self::write_rule(out, caps, rule_len)?;

        for row in &table.rows {
            for (pos, &i) in active_cols.iter().enumerate() {
                let cell = row.cells.get(i).map(|s| s.as_str()).unwrap_or("");
                let align = table
                    .alignments
                    .get(i)
                    .copied()
                    .unwrap_or(ColumnAlignment::Left);
                let col_width = widths.get(i).copied().unwrap_or(cell.width());
                let cell_truncated = truncate_with_ellipsis(cell, col_width, "...");
                let is_last = pos == active_cols.len() - 1;
                let formatted = if is_last && align == ColumnAlignment::Left {
                    cell_truncated
                } else {
                    super::terminal::pad_cell_for(&cell_truncated, col_width, align)
                };
                write!(out, "{formatted}")?;
                if !is_last {
                    write!(out, "{}", " ".repeat(col_gap))?;
                }
            }
            writeln!(out)?;
        }
        Ok(())
    }

    fn render_actions(
        actions: &[crate::semantic::Action],
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let gap = " ".repeat(style.action_gap(caps.is_narrow()));
        let (open_bracket, close_bracket) = style.key_delimiters();
        for (idx, action) in actions.iter().enumerate() {
            let key_str = action.trigger.display_tag();
            write!(
                out,
                "{open_bracket}{key_str}{close_bracket}{}{}",
                if action.label.is_empty() { "" } else { " " },
                action.label
            )?;
            if idx < actions.len() - 1 {
                write!(out, "{gap}")?;
            }
        }
        writeln!(out)
    }

    /// Pipe-safe status badge: ASCII glyph plus label, never any ANSI.
    pub fn write_status(out: &mut dyn Write, status: crate::semantic::Status) -> io::Result<()> {
        write!(out, "{} {}", status.ascii_glyph(), status.display_label())
    }

    fn write_rule(out: &mut dyn Write, caps: &Capabilities, len: usize) -> io::Result<()> {
        writeln!(out, "{}", "-".repeat(len.min(caps.width)))
    }

    fn write_heading_rule(
        out: &mut dyn Write,
        style: &ResolvedStyle,
        heading_width: usize,
    ) -> io::Result<()> {
        if style.section_rule == SectionRule::Short {
            writeln!(out, "{}", "-".repeat(heading_width))?;
        }
        Ok(())
    }
}
