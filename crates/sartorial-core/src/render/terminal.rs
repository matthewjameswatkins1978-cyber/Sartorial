use crate::capabilities::{Capabilities, SymbolMode};
use crate::document::{Block, Document};
use crate::preset::{BorderStyle, SectionRule, StatusLayout};
use crate::semantic::{ColumnAlignment, NoticeLevel, ProgressMode};
use crate::style::ResolvedStyle;
use crate::text::truncate_with_ellipsis;
use anstyle::Style;
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

/// Styled human terminal output. Consumes only resolved decisions from
/// [`ResolvedStyle`] plus [`Capabilities`]; it never reconstructs meaning.
pub struct TerminalRenderer;

impl TerminalRenderer {
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
                Self::write_gap(out, style, narrow)?;
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
                Self::render_title_line(text, version.as_deref(), style, out)?;
                Ok(())
            }
            Block::Subtitle { text } => {
                let indent = " ".repeat(style.title_marker.width());
                Self::write_styled(
                    out,
                    style.muted_style(),
                    &format!("{indent}{text}"),
                    style.color_enabled,
                )?;
                writeln!(out)
            }
            Block::StatusSection { label, status } => {
                Self::render_section(label, Some((*status, None)), style, caps, out)
            }
            Block::BadgeSection { label, badge } => {
                Self::render_badge_section(label, badge, style, caps, out)
            }
            Block::Summary { text } => {
                Self::write_styled(out, style.value_style(), text, style.color_enabled)?;
                writeln!(out)
            }
            Block::Facts { facts } => Self::render_facts(facts, style, caps, out),
            Block::Evidence { items } => {
                for ev in items {
                    if let Some(loc) = &ev.location {
                        Self::write_styled(out, style.muted_style(), loc, style.color_enabled)?;
                        writeln!(out)?;
                    }
                    Self::write_styled(out, style.value_style(), &ev.summary, style.color_enabled)?;
                    writeln!(out)?;
                }
                Ok(())
            }
            Block::EvidenceDetail {
                section_title,
                item,
            } => {
                if let Some(title) = section_title {
                    let heading = style.format_section_header(title);
                    let heading_len = heading.width();
                    Self::write_styled(out, style.section_style(), &heading, style.color_enabled)?;
                    writeln!(out)?;
                    Self::write_heading_rule(out, style, caps, heading_len)?;
                }
                if let Some(loc) = &item.location {
                    Self::write_styled(out, style.muted_style(), loc, style.color_enabled)?;
                    writeln!(out)?;
                }
                Self::write_styled(out, style.value_style(), &item.summary, style.color_enabled)?;
                writeln!(out)?;
                if let Some(handle) = &item.handle {
                    Self::write_styled(
                        out,
                        style.muted_style(),
                        &format!("Handle: {handle}"),
                        style.color_enabled,
                    )?;
                    writeln!(out)?;
                }
                if let Some(details) = &item.details {
                    Self::write_gap(out, style, caps.is_narrow())?;
                    Self::write_rule(out, style, caps, caps.width.min(60))?;
                    for line in details.lines() {
                        Self::write_styled(out, style.muted_style(), line, style.color_enabled)?;
                        writeln!(out)?;
                    }
                    Self::write_rule(out, style, caps, caps.width.min(60))?;
                }
                Ok(())
            }
            Block::Notices { notices } => {
                for notice in notices {
                    Self::render_notice(notice, style, caps, out)?;
                }
                Ok(())
            }
            Block::Table { table } => Self::render_table(table, style, caps, out),
            Block::Plan { plan } => Self::render_plan(plan, style, caps, out),
            Block::Receipt { receipt } => Self::render_receipt(receipt, style, caps, out),
            Block::Error { error } => Self::render_error(error, style, caps, out),
            Block::Actions { actions } => Self::render_actions(actions, style, caps, out),
            Block::Details { text } => {
                Self::write_styled(out, style.value_style(), text, style.color_enabled)?;
                writeln!(out)
            }
            Block::Choices { items } => {
                for item in items {
                    let tag = if item.disabled { "(unavailable)" } else { "" };
                    if tag.is_empty() {
                        writeln!(out, "  › {}", item.label)?;
                    } else {
                        Self::write_styled(
                            out,
                            style.muted_style(),
                            &format!("  › {} {tag}", item.label),
                            style.color_enabled,
                        )?;
                        writeln!(out)?;
                    }
                }
                Ok(())
            }
            Block::List { ordered, items } => {
                let unicode = caps_unicode(caps, style);
                for (idx, item) in items.iter().enumerate() {
                    if *ordered {
                        Self::write_styled(
                            out,
                            style.muted_style(),
                            &format!("{:2}. ", idx + 1),
                            style.color_enabled,
                        )?;
                    } else {
                        let bullet = if unicode { "• " } else { "- " };
                        Self::write_styled(out, style.muted_style(), bullet, style.color_enabled)?;
                    }
                    Self::write_styled(out, style.value_style(), item, style.color_enabled)?;
                    writeln!(out)?;
                }
                Ok(())
            }
            Block::ProgressSnapshot { state } => Self::render_progress(state, style, caps, out),
        }
    }

    // ---- primitives ----

    pub fn write_styled(
        out: &mut dyn Write,
        style: Style,
        text: &str,
        color_enabled: bool,
    ) -> io::Result<()> {
        if color_enabled {
            write!(out, "{style}{text}{style:#}")
        } else {
            write!(out, "{text}")
        }
    }

    fn write_gap(out: &mut dyn Write, style: &ResolvedStyle, narrow: bool) -> io::Result<()> {
        for _ in 0..style.component_gap(narrow) {
            writeln!(out)?;
        }
        Ok(())
    }

    fn write_rule(
        out: &mut dyn Write,
        style: &ResolvedStyle,
        caps: &Capabilities,
        len: usize,
    ) -> io::Result<()> {
        let line: String = std::iter::repeat_n(style.rule_char, len.min(caps.width)).collect();
        Self::write_styled(out, style.rule_style(), &line, style.color_enabled)?;
        writeln!(out)
    }

    pub fn write_status(
        out: &mut dyn Write,
        status: crate::semantic::Status,
        style: &ResolvedStyle,
        caps: &Capabilities,
    ) -> io::Result<()> {
        let glyph = match caps_unicode(caps, style) {
            true => status.unicode_glyph(),
            false => status.ascii_glyph(),
        };
        let label = status.display_label();
        if style.color_enabled {
            let s = style.status_style(status);
            write!(out, "{s}{glyph} {label}{s:#}")
        } else {
            write!(out, "{glyph} {label}")
        }
    }

    // ---- blocks ----

    fn render_title(
        text: &str,
        version: Option<&str>,
        subtitle: Option<&str>,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        Self::render_title_line(text, version, style, out)?;
        if let Some(sub) = subtitle {
            let indent = " ".repeat(style.title_marker.width());
            Self::write_styled(
                out,
                style.muted_style(),
                &format!("{indent}{sub}"),
                style.color_enabled,
            )?;
            writeln!(out)?;
        }
        if style.title_block_rule {
            Self::write_rule(out, style, caps, style.title_rule_len(caps.width))?;
        }
        Ok(())
    }

    /// Title with subtitle and grammar rule (Black Tie block rhythm).
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

    /// Bare title line with optional muted version (no block rule).
    pub fn render_title_line(
        text: &str,
        version: Option<&str>,
        style: &ResolvedStyle,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let title_text = style.format_title(text);
        Self::write_styled(out, style.title_style(), &title_text, style.color_enabled)?;
        if let Some(ver) = version {
            write!(out, " ")?;
            Self::write_styled(out, style.muted_style(), ver, style.color_enabled)?;
        }
        writeln!(out)
    }

    fn render_section(
        title: &str,
        status_or_badge: Option<(crate::semantic::Status, Option<String>)>,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let title_text = style.format_section_header(title);
        let title_len = title_text.width();
        Self::write_styled(out, style.section_style(), &title_text, style.color_enabled)?;

        let (status, badge) = match status_or_badge {
            Some((s, b)) => (Some(s), b),
            None => (None, None),
        };
        match (style.status_layout, status, badge) {
            (StatusLayout::Inline, Some(status), _) => {
                write!(out, "{}", " ".repeat(style.status_gap))?;
                Self::write_status(out, status, style, caps)?;
                writeln!(out)?;
            }
            (StatusLayout::Inline, None, Some(badge)) => {
                write!(out, "{}", " ".repeat(style.status_gap))?;
                Self::write_styled(out, style.muted_style(), &badge, style.color_enabled)?;
                writeln!(out)?;
            }
            (StatusLayout::Stacked, Some(status), _) => {
                writeln!(out)?;
                Self::write_heading_rule(out, style, caps, title_len)?;
                Self::write_status(out, status, style, caps)?;
                writeln!(out)?;
            }
            (StatusLayout::Stacked, None, Some(badge)) => {
                write!(out, "  ")?;
                Self::write_styled(out, style.muted_style(), &badge, style.color_enabled)?;
                writeln!(out)?;
            }
            (_, None, None) => {
                writeln!(out)?;
                Self::write_heading_rule(out, style, caps, title_len)?;
            }
        }
        Ok(())
    }

    fn write_heading_rule(
        out: &mut dyn Write,
        style: &ResolvedStyle,
        caps: &Capabilities,
        heading_width: usize,
    ) -> io::Result<()> {
        if style.section_rule == SectionRule::Short {
            Self::write_rule(out, style, caps, heading_width)?;
        }
        Ok(())
    }

    fn render_facts(
        facts: &[crate::semantic::Fact],
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        if facts.is_empty() {
            return Ok(());
        }
        let narrow = caps.is_narrow();
        let labels: Vec<String> = facts
            .iter()
            .map(|f| style.format_fact_label(&f.name))
            .collect();
        let max_label_len = labels.iter().map(|l| l.width()).max().unwrap_or(0);
        let pad_spacing = style.fact_pad(narrow);
        let pad_label = (max_label_len + pad_spacing).min(30);

        for (fact, label) in facts.iter().zip(labels.iter()) {
            if narrow {
                Self::write_styled(out, style.label_style(), label, style.color_enabled)?;
                if !label.ends_with(':') {
                    write!(out, ":")?;
                }
                writeln!(out)?;
                write!(out, "  ")?;
                let val_style = if fact.muted {
                    style.muted_style()
                } else {
                    style.value_style()
                };
                Self::write_styled(out, val_style, &fact.value, style.color_enabled)?;
                if let Some(unit) = &fact.unit {
                    write!(out, " ")?;
                    Self::write_styled(out, style.muted_style(), unit, style.color_enabled)?;
                }
                writeln!(out)?;
            } else {
                let label_width = label.width();
                Self::write_styled(out, style.label_style(), label, style.color_enabled)?;
                let pad = if pad_label > label_width {
                    pad_label - label_width
                } else {
                    pad_spacing
                };
                write!(out, "{}", " ".repeat(pad))?;
                let val_style = if fact.muted {
                    style.muted_style()
                } else {
                    style.value_style()
                };
                Self::write_styled(out, val_style, &fact.value, style.color_enabled)?;
                if let Some(unit) = &fact.unit {
                    write!(out, " ")?;
                    Self::write_styled(out, style.muted_style(), unit, style.color_enabled)?;
                }
                writeln!(out)?;
            }
        }
        Ok(())
    }

    fn render_notice(
        notice: &crate::semantic::Notice,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let unicode = caps_unicode(caps, style);
        let glyph = if unicode {
            notice.level.unicode_glyph()
        } else {
            notice.level.ascii_glyph()
        };

        if style.notice_compact {
            Self::write_styled(
                out,
                style.notice_style(notice.level),
                glyph,
                style.color_enabled,
            )?;
            write!(out, " {}", notice.message)?;
            if let Some(detail) = &notice.detail {
                let sep = if unicode { " · " } else { " - " };
                Self::write_styled(
                    out,
                    style.muted_style(),
                    &format!("{sep}{detail}"),
                    style.color_enabled,
                )?;
            }
            return writeln!(out);
        }

        if notice.level == NoticeLevel::Info {
            Self::write_styled(
                out,
                style.muted_style(),
                &notice.message,
                style.color_enabled,
            )?;
        } else {
            Self::write_styled(
                out,
                style.notice_style(notice.level),
                glyph,
                style.color_enabled,
            )?;
            write!(out, " ")?;
            Self::write_styled(
                out,
                style.value_style(),
                &notice.message,
                style.color_enabled,
            )?;
        }

        if let Some(detail) = &notice.detail {
            write!(out, " ")?;
            Self::write_styled(out, style.muted_style(), detail, style.color_enabled)?;
        }
        writeln!(out)
    }

    fn render_table(
        table: &crate::semantic::TableModel,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        // v0.2 TableView rhythm: Section(title).with_badge(badge), then grid.
        if let Some(title) = &table.title {
            let badge = table.badge.clone().unwrap_or_default();
            Self::render_badge_section(title, &badge, style, caps, out)?;
        }
        Self::render_table_rows(table, style, caps, out, false)
    }

    /// Section heading with raw badge text (no status glyph):
    /// `Collection` + count, table titles + badges.
    fn render_badge_section(
        label: &str,
        badge: &str,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let title_text = style.format_section_header(label);
        if badge.is_empty() {
            // Plain heading: same rhythm as a status-less section.
            Self::write_styled(out, style.section_style(), &title_text, style.color_enabled)?;
            writeln!(out)?;
            Self::write_heading_rule(out, style, caps, title_text.width())?;
            return Ok(());
        }
        Self::write_styled(out, style.section_style(), &title_text, style.color_enabled)?;
        match style.status_layout {
            StatusLayout::Inline => {
                write!(out, "{}", " ".repeat(style.status_gap))?;
                Self::write_styled(out, style.muted_style(), badge, style.color_enabled)?;
                writeln!(out)?;
            }
            StatusLayout::Stacked => {
                write!(out, "  ")?;
                Self::write_styled(out, style.muted_style(), badge, style.color_enabled)?;
                writeln!(out)?;
            }
        }
        Ok(())
    }

    fn render_table_rows(
        table: &crate::semantic::TableModel,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
        _plain: bool,
    ) -> io::Result<()> {
        let narrow = caps.is_narrow();
        let base_col_gap = style.table_col_gap(narrow);
        let (widths, col_gap) = calculate_column_widths(table, caps.width, base_col_gap);
        let active_cols: Vec<usize> = (0..widths.len()).filter(|&i| widths[i] > 0).collect();
        if active_cols.is_empty() {
            return Ok(());
        }
        let total_content_width =
            widths.iter().sum::<usize>() + (active_cols.len().saturating_sub(1) * col_gap);
        let rule_len = total_content_width.min(caps.width);
        let unicode = caps_unicode(caps, style);

        if style.table_headers && !table.headers.is_empty() {
            for (pos, &i) in active_cols.iter().enumerate() {
                let cell = table.headers.get(i).map(|s| s.as_str()).unwrap_or("");
                let col_width = widths.get(i).copied().unwrap_or(cell.width());
                let cell_truncated =
                    truncate_with_ellipsis(cell, col_width, if unicode { "…" } else { "..." });
                let is_last = pos == active_cols.len() - 1;
                let formatted = if is_last {
                    cell_truncated
                } else {
                    pad_cell(&cell_truncated, col_width, ColumnAlignment::Left)
                };
                Self::write_styled(out, style.section_style(), &formatted, style.color_enabled)?;
                if !is_last {
                    write!(out, "{}", " ".repeat(col_gap))?;
                }
            }
            writeln!(out)?;
        }

        if style.border == BorderStyle::Subtle {
            Self::write_rule(out, style, caps, rule_len)?;
        }

        for row in &table.rows {
            for (pos, &i) in active_cols.iter().enumerate() {
                let cell = row.cells.get(i).map(|s| s.as_str()).unwrap_or("");
                let align = table
                    .alignments
                    .get(i)
                    .copied()
                    .unwrap_or(ColumnAlignment::Left);
                let col_width = widths.get(i).copied().unwrap_or(cell.width());
                let cell_truncated =
                    truncate_with_ellipsis(cell, col_width, if unicode { "…" } else { "..." });
                let is_last = pos == active_cols.len() - 1;
                let formatted = if is_last && align == ColumnAlignment::Left {
                    cell_truncated
                } else {
                    pad_cell(&cell_truncated, col_width, align)
                };

                if is_last && matches!(cell, "ready" | "missing" | "failed" | "attention") {
                    let status = match cell {
                        "ready" => crate::semantic::Status::Ready,
                        "failed" | "missing" => crate::semantic::Status::Failed,
                        _ => crate::semantic::Status::Attention,
                    };
                    Self::write_styled(
                        out,
                        style.status_style(status),
                        &formatted,
                        style.color_enabled,
                    )?;
                } else {
                    Self::write_styled(out, style.value_style(), &formatted, style.color_enabled)?;
                }

                if !is_last {
                    write!(out, "{}", " ".repeat(col_gap))?;
                }
            }
            writeln!(out)?;
        }
        Ok(())
    }

    fn render_plan(
        plan: &crate::semantic::Plan,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        if let Some(desc) = &plan.description {
            Self::write_styled(out, style.section_style(), desc, style.color_enabled)?;
            writeln!(out)?;
            writeln!(out)?;
        }

        for change in &plan.changes {
            let symbol = change.kind.symbol();
            if style.color_enabled {
                Self::write_styled(out, style.change_style(change.kind), symbol, true)?;
            } else {
                write!(out, "{symbol}")?;
            }
            write!(out, " ")?;
            Self::write_styled(
                out,
                style.value_style(),
                &change.target,
                style.color_enabled,
            )?;
            writeln!(out)?;
            if let Some(detail) = &change.detail {
                write!(out, "  ")?;
                Self::write_styled(out, style.muted_style(), detail, style.color_enabled)?;
                writeln!(out)?;
            }
        }

        if !plan.changes.is_empty() && (!plan.consequences.is_empty() || !plan.warnings.is_empty())
        {
            writeln!(out)?;
        }

        let unicode = caps_unicode(caps, style);
        for warn in &plan.warnings {
            let prefix = if unicode { "! " } else { "[!] " };
            if style.color_enabled {
                Self::write_styled(
                    out,
                    style.notice_style(crate::semantic::NoticeLevel::Warning),
                    prefix,
                    true,
                )?;
                Self::write_styled(
                    out,
                    style.notice_style(crate::semantic::NoticeLevel::Warning),
                    warn,
                    true,
                )?;
            } else {
                write!(out, "{prefix}{warn}")?;
            }
            writeln!(out)?;
        }

        for c in &plan.consequences {
            Self::write_styled(out, style.muted_style(), c, style.color_enabled)?;
            writeln!(out)?;
        }

        if let Some(rev) = plan.reversible {
            let rev_text = if rev {
                "Reversible: Yes (can be rolled back)"
            } else {
                "Reversible: No (irreversible operation)"
            };
            Self::write_styled(out, style.muted_style(), rev_text, style.color_enabled)?;
            writeln!(out)?;
        }

        if !plan.actions.is_empty() {
            writeln!(out)?;
            Self::render_actions(&plan.actions, style, caps, out)?;
        }
        Ok(())
    }

    fn render_receipt(
        receipt: &crate::semantic::Receipt,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        Self::write_status(out, receipt.status, style, caps)?;
        write!(out, " ")?;
        Self::write_styled(
            out,
            style.title_style(),
            &receipt.title,
            style.color_enabled,
        )?;
        writeln!(out)?;
        writeln!(out)?;

        if !receipt.changes.is_empty() {
            Self::render_facts(&receipt.changes, style, caps, out)?;
        }

        if !receipt.unchanged.is_empty() {
            writeln!(out)?;
            Self::write_styled(out, style.muted_style(), "Unchanged:", style.color_enabled)?;
            writeln!(out)?;
            Self::render_facts(&receipt.unchanged, style, caps, out)?;
        }

        for warn in &receipt.warnings {
            writeln!(out)?;
            Self::render_notice(warn, style, caps, out)?;
        }

        if let Some(guidance) = &receipt.guidance {
            writeln!(out)?;
            Self::write_styled(out, style.value_style(), guidance, style.color_enabled)?;
            writeln!(out)?;
        }

        if let Some(handle) = &receipt.evidence_handle {
            writeln!(out)?;
            Self::write_styled(
                out,
                style.muted_style(),
                &format!("Evidence reference: {handle}"),
                style.color_enabled,
            )?;
            writeln!(out)?;
        }

        if !receipt.actions.is_empty() {
            writeln!(out)?;
            Self::render_actions(&receipt.actions, style, caps, out)?;
        }
        Ok(())
    }

    fn render_error(
        error: &crate::semantic::ErrorModel,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        let what = match style.title_case {
            crate::preset::TitleCase::Upper => error.what.to_uppercase(),
            crate::preset::TitleCase::Preserve => error.what.clone(),
        };
        Self::write_styled(
            out,
            style.status_style(crate::semantic::Status::Failed),
            &what,
            style.color_enabled,
        )?;
        writeln!(out)?;
        writeln!(out)?;

        if let Some(why) = &error.why {
            Self::write_styled(out, style.value_style(), why, style.color_enabled)?;
            writeln!(out)?;
        } else {
            Self::write_styled(
                out,
                style.muted_style(),
                "Cause undetermined (no conclusive root cause established).",
                style.color_enabled,
            )?;
            writeln!(out)?;
        }

        if !error.evidence.is_empty() {
            writeln!(out)?;
            for ev in &error.evidence {
                if let Some(loc) = &ev.location {
                    Self::write_styled(out, style.muted_style(), loc, style.color_enabled)?;
                    writeln!(out)?;
                }
                Self::write_styled(out, style.value_style(), &ev.summary, style.color_enabled)?;
                writeln!(out)?;
                if let Some(handle) = &ev.handle {
                    Self::write_styled(
                        out,
                        style.muted_style(),
                        &format!("Ref: {handle}"),
                        style.color_enabled,
                    )?;
                    writeln!(out)?;
                }
            }
        }

        if !error.next_actions.is_empty() {
            writeln!(out)?;
            Self::render_actions(&error.next_actions, style, caps, out)?;
        }
        Ok(())
    }

    fn render_actions(
        actions: &[crate::semantic::Action],
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        if actions.is_empty() {
            return Ok(());
        }
        let gap = " ".repeat(style.action_gap(caps.is_narrow()));
        let (open_bracket, close_bracket) = style.key_delimiters();
        for (idx, action) in actions.iter().enumerate() {
            let key_str = action.trigger.display_tag();
            Self::write_styled(
                out,
                style.key_bracket_style(),
                open_bracket,
                style.color_enabled,
            )?;
            Self::write_styled(out, style.key_char_style(), &key_str, style.color_enabled)?;
            Self::write_styled(
                out,
                style.key_bracket_style(),
                close_bracket,
                style.color_enabled,
            )?;
            write!(out, " ")?;
            Self::write_styled(out, style.value_style(), &action.label, style.color_enabled)?;
            if idx < actions.len() - 1 {
                write!(out, "{gap}")?;
            }
        }
        writeln!(out)
    }

    fn render_progress(
        state: &crate::semantic::ProgressState,
        style: &ResolvedStyle,
        caps: &Capabilities,
        out: &mut dyn Write,
    ) -> io::Result<()> {
        // Static snapshot only: honest counts, derived percent, never invented.
        let _ = caps;
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
        let _ = style.progress_treatment;
        Self::write_styled(out, style.value_style(), &line, style.color_enabled)?;
        writeln!(out)
    }
}

fn caps_unicode(caps: &Capabilities, style: &ResolvedStyle) -> bool {
    style.symbols != SymbolMode::Ascii && caps.unicode
}

pub(crate) fn column_widths_for(
    table: &crate::semantic::TableModel,
    max_terminal_width: usize,
    col_gap: usize,
) -> (Vec<usize>, usize) {
    calculate_column_widths(table, max_terminal_width, col_gap)
}

pub(crate) fn pad_cell_for(text: &str, width: usize, alignment: ColumnAlignment) -> String {
    pad_cell(text, width, alignment)
}

fn calculate_column_widths(
    table: &crate::semantic::TableModel,
    max_terminal_width: usize,
    col_gap: usize,
) -> (Vec<usize>, usize) {
    let col_count = table.headers.len();
    if col_count == 0 || max_terminal_width == 0 {
        return (Vec::new(), 0);
    }
    let actual_gap = if max_terminal_width < 30 {
        1.min(col_gap)
    } else {
        col_gap
    };
    let mut widths = vec![0; col_count];
    for (i, h) in table.headers.iter().enumerate() {
        widths[i] = widths[i].max(h.width());
    }
    for row in &table.rows {
        for (i, cell) in row.cells.iter().enumerate() {
            if i < col_count {
                widths[i] = widths[i].max(cell.width());
            }
        }
    }
    let total_gaps = col_count.saturating_sub(1) * actual_gap;
    let available_width = max_terminal_width.saturating_sub(total_gaps);
    let mut current_total: usize = widths.iter().sum();
    while current_total > available_width {
        let (max_idx, &max_val) = widths.iter().enumerate().max_by_key(|(_, &w)| w).unwrap();
        if max_val <= 1 {
            break;
        }
        widths[max_idx] -= 1;
        current_total -= 1;
    }
    let mut total_with_gaps = widths.iter().filter(|&&w| w > 0).sum::<usize>()
        + (widths.iter().filter(|&&w| w > 0).count().saturating_sub(1) * actual_gap);
    if total_with_gaps > max_terminal_width {
        for i in (0..col_count).rev() {
            if total_with_gaps <= max_terminal_width {
                break;
            }
            widths[i] = 0;
            let active = widths.iter().filter(|&&w| w > 0).count();
            total_with_gaps =
                widths.iter().sum::<usize>() + (active.saturating_sub(1) * actual_gap);
        }
        if widths.iter().any(|&w| w > 0) && widths[0] > max_terminal_width {
            widths[0] = max_terminal_width;
        }
    }
    (widths, actual_gap)
}

fn pad_cell(text: &str, width: usize, alignment: ColumnAlignment) -> String {
    let text_width = text.width();
    if text_width >= width {
        return text.to_string();
    }
    let diff = width - text_width;
    match alignment {
        ColumnAlignment::Left => format!("{text}{}", " ".repeat(diff)),
        ColumnAlignment::Right => format!("{}{text}", " ".repeat(diff)),
        ColumnAlignment::Center => {
            let left_pad = diff / 2;
            let right_pad = diff - left_pad;
            format!("{}{text}{}", " ".repeat(left_pad), " ".repeat(right_pad))
        }
    }
}
