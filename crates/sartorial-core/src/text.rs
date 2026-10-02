use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Terminal display width of a string per Unicode rules.
pub fn display_width(text: &str) -> usize {
    text.width()
}

/// Truncate to `max_width` display columns, appending an ellipsis if truncated.
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

/// Truncate a path/identifier from the middle, preserving prefix and filename.
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

/// Truncate long text preserving head and tail around a middle ellipsis.
pub fn truncate_middle(text: &str, max_width: usize, ellipsis: &str) -> String {
    truncate_path(text, max_width, ellipsis)
}

/// Wrap words to `max_width` display columns. Words longer than the width
/// are hard-split. Never panics on tiny widths.
pub fn wrap_words(text: &str, max_width: usize) -> Vec<String> {
    let max_width = max_width.max(1);
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        if paragraph.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        let mut current = String::new();
        let mut current_w = 0;
        for word in paragraph.split_whitespace() {
            let word_w = word.width();
            if word_w > max_width {
                if !current.is_empty() {
                    lines.push(std::mem::take(&mut current));
                    current_w = 0;
                }
                let mut chunk = String::new();
                let mut chunk_w = 0;
                for ch in word.chars() {
                    let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0);
                    if chunk_w + ch_w > max_width {
                        lines.push(std::mem::take(&mut chunk));
                        chunk_w = 0;
                    }
                    chunk_w += ch_w;
                    chunk.push(ch);
                }
                if !chunk.is_empty() {
                    current = chunk;
                    current_w = chunk_w;
                }
                continue;
            }
            let gap = if current.is_empty() { 0 } else { 1 };
            if current_w + gap + word_w > max_width {
                lines.push(std::mem::take(&mut current));
                current_w = 0;
            }
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
            current_w += gap + word_w;
        }
        lines.push(current);
    }
    lines
}
