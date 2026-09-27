use crate::components::section::Section;
use crate::config::{BorderStyle, SymbolMode};
use crate::render::context::RenderContext;
use crate::render::human::HumanRenderer;
use crate::render::plain::PlainRenderer;
use crate::render::{RenderHuman, RenderPlain};
use crate::semantic::table::{ColumnAlignment, TableModel};
use std::io::{self, Write};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Table component following the restrained Biscuit Logic visual standard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableView {
    pub model: TableModel,
}

impl TableView {
    pub fn new(model: TableModel) -> Self {
        Self { model }
    }

    /// Truncate text to fit within max_width display cells using ellipsis.
    pub fn truncate_with_ellipsis(text: &str, max_width: usize, is_ascii: bool) -> String {
        let current_width = text.width();
        if current_width <= max_width {
            return text.to_string();
        }
        let ellipsis = if is_ascii { "..." } else { "…" };
        let ellipsis_width = ellipsis.width();
        if max_width <= ellipsis_width {
            return ellipsis.chars().take(max_width).collect();
        }
        let target_width = max_width - ellipsis_width;
        let mut res = String::new();
        let mut accumulated = 0;
        for ch in text.chars() {
            let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);
            if accumulated + ch_w > target_width {
                break;
            }
            accumulated += ch_w;
            res.push(ch);
        }
        res.push_str(ellipsis);
        res
    }

    /// Calculate column widths bounded by terminal width.
    fn calculate_column_widths(&self, max_terminal_width: usize, col_gap: usize) -> Vec<usize> {
        let col_count = self.model.headers.len();
        if col_count == 0 {
            return Vec::new();
        }
        let mut widths = vec![0; col_count];

        for (i, h) in self.model.headers.iter().enumerate() {
            widths[i] = widths[i].max(h.width());
        }

        for row in &self.model.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                if i < col_count {
                    widths[i] = widths[i].max(cell.width());
                }
            }
        }

        // Enforce terminal width constraint
        let total_gaps = col_count.saturating_sub(1) * col_gap;
        let available_width = max_terminal_width.saturating_sub(total_gaps);

        let mut current_total: usize = widths.iter().sum();
        while current_total > available_width {
            let (max_idx, &max_val) = widths.iter().enumerate().max_by_key(|(_, &w)| w).unwrap();
            if max_val <= 3 {
                break;
            }
            widths[max_idx] -= 1;
            current_total -= 1;
        }

        widths
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
}

impl RenderHuman for TableView {
    fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        if let Some(ref title) = self.model.title {
            let mut sec = Section::new(title);
            if let Some(ref badge) = self.model.badge {
                sec = sec.with_badge(badge);
            }
            sec.render_human(ctx, out)?;
        }

        let col_gap = if ctx.is_narrow() { 2 } else { 4 };
        let widths = self.calculate_column_widths(ctx.width, col_gap);
        let total_content_width =
            widths.iter().sum::<usize>() + (widths.len().saturating_sub(1) * col_gap);
        let rule_len = total_content_width.min(ctx.width);

        if ctx.config.border == BorderStyle::Subtle {
            HumanRenderer::write_rule(out, ctx, rule_len)?;
        }

        let is_ascii = ctx.symbols == SymbolMode::Ascii;

        // Render rows
        for row in &self.model.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                let align = self
                    .model
                    .alignments
                    .get(i)
                    .copied()
                    .unwrap_or(ColumnAlignment::Left);
                let col_width = widths.get(i).copied().unwrap_or(cell.width());
                let cell_truncated = Self::truncate_with_ellipsis(cell, col_width, is_ascii);
                let is_last = i == row.cells.len() - 1;
                let formatted = if is_last && align == ColumnAlignment::Left {
                    cell_truncated
                } else {
                    Self::pad_cell(&cell_truncated, col_width, align)
                };

                // Last column has semantic status styling if recognized
                if is_last
                    && (cell == "ready"
                        || cell == "missing"
                        || cell == "failed"
                        || cell == "attention")
                {
                    let style =
                        match cell.as_str() {
                            "ready" => anstyle::Style::new()
                                .fg_color(Some(anstyle::AnsiColor::Green.into())),
                            "missing" | "failed" => {
                                anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Red.into()))
                            }
                            "attention" => anstyle::Style::new()
                                .fg_color(Some(anstyle::AnsiColor::Yellow.into())),
                            _ => HumanRenderer::value_style(),
                        };
                    HumanRenderer::write_styled(out, style, &formatted, ctx.color_enabled)?;
                } else {
                    HumanRenderer::write_styled(
                        out,
                        HumanRenderer::value_style(),
                        &formatted,
                        ctx.color_enabled,
                    )?;
                }

                if i < row.cells.len() - 1 {
                    write!(out, "{}", " ".repeat(col_gap))?;
                }
            }
            writeln!(out)?;
        }

        Ok(())
    }
}

impl RenderPlain for TableView {
    fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
        if let Some(ref title) = self.model.title {
            let mut sec = Section::new(title);
            if let Some(ref badge) = self.model.badge {
                sec = sec.with_badge(badge);
            }
            sec.render_plain(ctx, out)?;
        }

        let col_gap = if ctx.is_narrow() { 2 } else { 4 };
        let widths = self.calculate_column_widths(ctx.width, col_gap);
        let total_content_width =
            widths.iter().sum::<usize>() + (widths.len().saturating_sub(1) * col_gap);
        let rule_len = total_content_width.min(ctx.width);

        PlainRenderer::write_rule(out, ctx, rule_len)?;

        let is_ascii = ctx.symbols == SymbolMode::Ascii;

        for row in &self.model.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                let align = self
                    .model
                    .alignments
                    .get(i)
                    .copied()
                    .unwrap_or(ColumnAlignment::Left);
                let col_width = widths.get(i).copied().unwrap_or(cell.width());
                let cell_truncated = Self::truncate_with_ellipsis(cell, col_width, is_ascii);
                let is_last = i == row.cells.len() - 1;
                let formatted = if is_last && align == ColumnAlignment::Left {
                    cell_truncated
                } else {
                    Self::pad_cell(&cell_truncated, col_width, align)
                };

                write!(out, "{formatted}")?;
                if i < row.cells.len() - 1 {
                    write!(out, "{}", " ".repeat(col_gap))?;
                }
            }
            writeln!(out)?;
        }

        Ok(())
    }
}
