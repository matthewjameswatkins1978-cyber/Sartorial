use crate::semantic::{
    Action, ChoiceItem, ErrorModel, Evidence, Fact, Notice, Outcome, Plan, ProgressState, Receipt,
    Status, TableModel,
};
use serde::{Deserialize, Serialize};

/// Central presentation representation.
///
/// Meaning is resolved before renderer-specific formatting: semantic
/// objects convert into a `Document` once, and the terminal / plain /
/// Markdown renderers only decide spacing, wrapping, glyphs, and emphasis.
/// A renderer never decides whether something succeeded, what evidence
/// means, or what changed.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Document {
    pub blocks: Vec<Block>,
}

impl Document {
    pub fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    pub fn push(mut self, block: Block) -> Self {
        self.blocks.push(block);
        self
    }

    pub fn title(text: impl Into<String>) -> Self {
        Self::new().push(Block::Title {
            text: text.into(),
            version: None,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}

/// One resolved presentation unit. All application meaning is already
/// decided; only visual treatment is left to the renderer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Block {
    Title {
        text: String,
        /// Optional version string riding the title line (muted, un-cased).
        version: Option<String>,
    },
    Subtitle {
        text: String,
    },
    /// Labelled status section (e.g. `Status    [OK] READY`).
    StatusSection {
        label: String,
        status: Status,
    },
    /// Labelled badge section (e.g. `Collection    11 items`).
    BadgeSection {
        label: String,
        badge: String,
    },
    Summary {
        text: String,
    },
    Facts {
        facts: Vec<Fact>,
    },
    Evidence {
        items: Vec<Evidence>,
    },
    /// Full evidence disclosure: optional section heading, location,
    /// summary, handle, and details between rules. Used by detail views;
    /// compact [`Block::Evidence`] stays the outcome voice.
    EvidenceDetail {
        section_title: Option<String>,
        item: Evidence,
    },
    Notices {
        notices: Vec<Notice>,
    },
    Table {
        table: TableModel,
    },
    Plan {
        plan: Plan,
    },
    Receipt {
        receipt: Receipt,
    },
    Error {
        error: ErrorModel,
    },
    Actions {
        actions: Vec<Action>,
    },
    Details {
        text: String,
    },
    Choices {
        items: Vec<ChoiceItem>,
    },
    /// Generic ordered or bulleted list of plain strings.
    List {
        ordered: bool,
        items: Vec<String>,
    },
    /// Static snapshot of in-flight progress (no animation; live progress
    /// belongs to the batteries-included crate).
    ProgressSnapshot {
        state: ProgressState,
    },
}

/// Small abstraction converging rendering through one path: every semantic
/// object produces a [`Document`], renderers consume only documents.
pub trait Presentable {
    fn to_document(&self) -> Document;
}

impl Presentable for Document {
    fn to_document(&self) -> Document {
        self.clone()
    }
}

impl Presentable for Outcome {
    fn to_document(&self) -> Document {
        let mut doc = Document::new().push(Block::Title {
            text: self.title.clone(),
            version: None,
        });
        doc.blocks.push(Block::StatusSection {
            label: "Status".to_string(),
            status: self.status,
        });
        if let Some(summary) = &self.summary {
            doc.blocks.push(Block::Summary {
                text: summary.clone(),
            });
        }
        if !self.facts.is_empty() {
            doc.blocks.push(Block::Facts {
                facts: self.facts.clone(),
            });
        }
        if !self.evidence.is_empty() {
            doc.blocks.push(Block::Evidence {
                items: self.evidence.clone(),
            });
        }
        if !self.warnings.is_empty() {
            doc.blocks.push(Block::Notices {
                notices: self.warnings.clone(),
            });
        }
        if !self.actions.is_empty() {
            doc.blocks.push(Block::Actions {
                actions: self.actions.clone(),
            });
        }
        if let Some(details) = &self.details {
            doc.blocks.push(Block::Details {
                text: details.clone(),
            });
        }
        doc
    }
}

impl Presentable for Receipt {
    fn to_document(&self) -> Document {
        // Receipts carry their own header line (badge + title), mirroring
        // the v0.2 receipt composition.
        Document::new().push(Block::Receipt {
            receipt: self.clone(),
        })
    }
}

impl Presentable for Plan {
    fn to_document(&self) -> Document {
        Document::new()
            .push(Block::Title {
                text: self.title.clone(),
                version: None,
            })
            .push(Block::Plan { plan: self.clone() })
    }
}

impl Presentable for ErrorModel {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Error {
            error: self.clone(),
        })
    }
}

impl Presentable for TableModel {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Table {
            table: self.clone(),
        })
    }
}

impl Presentable for Notice {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Notices {
            notices: vec![self.clone()],
        })
    }
}

impl Presentable for ProgressState {
    fn to_document(&self) -> Document {
        Document::new().push(Block::ProgressSnapshot {
            state: self.clone(),
        })
    }
}

impl Presentable for Vec<Fact> {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Facts {
            facts: self.clone(),
        })
    }
}

impl Presentable for Vec<Action> {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Actions {
            actions: self.clone(),
        })
    }
}

impl Presentable for Vec<ChoiceItem> {
    fn to_document(&self) -> Document {
        Document::new().push(Block::Choices {
            items: self.clone(),
        })
    }
}
