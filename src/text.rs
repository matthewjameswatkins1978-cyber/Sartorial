use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Measures the terminal display width of a string accurately according to Unicode rules.
pub fn display_width(text: &str) -> usize {
    text.width()
}

/// Truncates a string to fit within `max_width` display columns, appending an ellipsis if truncated.
pub fn truncate_with_ellipsis(text: &str, max_width: usize, ellipsis: &str) -> String {
    let current_width = text.width();
    if current_width <= max_width {
        return text.to_string();
    }
    let ellipsis_w = ellipsis.width();
    if max_width <= ellipsis_w {
        return ellipsis.chars().take(max_width).collect();
    }
    let target_width = max_width - ellipsis_w;
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

/// Truncates a path or identifier from the middle, preserving both important prefix and filename/suffix.
/// E.g. `C:\Users\Matmus\...\threadmoth.exe`
pub fn truncate_path(path: &str, max_width: usize, ellipsis: &str) -> String {
    if path.width() <= max_width {
        return path.to_string();
    }
    let ellipsis_w = ellipsis.width();
    if max_width <= ellipsis_w + 4 {
        return truncate_with_ellipsis(path, max_width, ellipsis);
    }

    let target_content = max_width - ellipsis_w;
    let head_target = target_content / 2;
    let tail_target = target_content - head_target;

    // Collect head
    let mut head = String::new();
    let mut accumulated = 0;
    for ch in path.chars() {
        let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);
        if accumulated + ch_w > head_target {
            break;
        }
        accumulated += ch_w;
        head.push(ch);
    }

    // Collect tail
    let mut tail = Vec::new();
    let mut accumulated_tail = 0;
    for ch in path.chars().rev() {
        let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);
        if accumulated_tail + ch_w > tail_target {
            break;
        }
        accumulated_tail += ch_w;
        tail.push(ch);
    }
    tail.reverse();
    let tail_str: String = tail.into_iter().collect();

    format!("{head}{ellipsis}{tail_str}")
}

/// Truncates a long text string preserving head and tail around a middle ellipsis.
pub fn truncate_middle(text: &str, max_width: usize, ellipsis: &str) -> String {
    truncate_path(text, max_width, ellipsis)
}
