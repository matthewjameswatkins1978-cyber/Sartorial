//! Convergence adapters: every semantic object becomes a [`Document`]
//! once, then the terminal / plain / Markdown renderers consume only
//! the document. Compatibility [`RenderHuman`] / [`RenderPlain`] impls
//! live here so rendering converges instead of drifting per type.

use crate::render::context::RenderContext;
use crate::render::{RenderHuman, RenderPlain};
use sartorial_core::render::{MarkdownRenderer, PlainRenderer, TerminalRenderer};
use sartorial_core::{Document, Presentable};
use std::io::{self, Write};

/// First-class Markdown rendering for any [`Presentable`] value.
pub trait RenderMarkdown {
    fn render_markdown(&self, ctx: &RenderContext) -> io::Result<String>;
}

impl<T: Presentable> RenderMarkdown for T {
    fn render_markdown(&self, ctx: &RenderContext) -> io::Result<String> {
        let doc = self.to_document();
        MarkdownRenderer::render_to_string(&doc, &ctx.style, &ctx.caps)
    }
}

macro_rules! converge {
    ($($ty:ty),*) => {
        $(
            impl RenderHuman for $ty {
                fn render_human(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
                    let doc: Document = self.to_document();
                    TerminalRenderer::render(&doc, &ctx.style, &ctx.caps, out)
                }
            }

            impl RenderPlain for $ty {
                fn render_plain(&self, ctx: &RenderContext, out: &mut dyn Write) -> io::Result<()> {
                    let doc: Document = self.to_document();
                    PlainRenderer::render(&doc, &ctx.style, &ctx.caps, out)
                }
            }
        )*
    };
}

converge!(
    sartorial_core::Outcome,
    sartorial_core::ErrorModel,
    sartorial_core::TableModel,
    sartorial_core::Notice,
    sartorial_core::ProgressState,
    sartorial_core::Document,
    Vec<sartorial_core::Fact>,
    Vec<sartorial_core::Action>,
    Vec<sartorial_core::ChoiceItem>
);

// `Plan` and `Receipt` converge through the same path; their compatibility
// impls live beside the component shims (`components::plan`,
// `components::receipt`) to keep one reviewable owner per type.
