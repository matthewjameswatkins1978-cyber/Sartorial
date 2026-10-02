use serde::{Deserialize, Serialize};

/// Text alignment within a table column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnAlignment {
    #[default]
    Left,
    Center,
    Right,
}

/// A row in a structured table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<String>,
}

impl TableRow {
    pub fn new(cells: Vec<String>) -> Self {
        Self { cells }
    }
}

impl<S: Into<String>> FromIterator<S> for TableRow {
    fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
        Self {
            cells: iter.into_iter().map(Into::into).collect(),
        }
    }
}

/// A strongly typed table model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
    pub headers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alignments: Vec<ColumnAlignment>,
    pub rows: Vec<TableRow>,
}

impl TableModel {
    pub fn new(headers: Vec<impl Into<String>>) -> Self {
        let headers: Vec<String> = headers.into_iter().map(Into::into).collect();
        let alignments = vec![ColumnAlignment::Left; headers.len()];
        Self {
            title: None,
            badge: None,
            headers,
            alignments,
            rows: Vec::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    pub fn with_alignments(mut self, alignments: Vec<ColumnAlignment>) -> Self {
        self.alignments = alignments;
        self
    }

    pub fn add_row(&mut self, row: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.rows.push(TableRow::from_iter(row));
        self
    }

    pub fn with_row(mut self, row: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.add_row(row);
        self
    }
}
