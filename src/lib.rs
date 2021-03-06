/*!
This crate provides a *very* simple markdown parser.

Its main motivation was to be the basis of the [termimad](https://github.com/Canop/termimad) lib, which displays static and dynamic markdown snippets on a terminal without mixing the skin with the code and wrapping the text and tables as needed.

It can be used on its own:

```rust
use minimad::*;

assert_eq!(
    parse_line("## a header with some **bold**!"),
    Line::new_header(
        2,
        vec![
            Compound::raw_str("a header with some "),
            Compound::raw_str("bold").bold(),
            Compound::raw_str("!"),
        ]
    )
);

assert_eq!(
    parse_inline("*Italic then **bold and italic `and some *code*`** and italic*"),
    Composite::from(vec![
        Compound::raw_str("Italic then ").italic(),
        Compound::raw_str("bold and italic ").bold().italic(),
        Compound::raw_str("and some *code*").bold().italic().code(),
        Compound::raw_str(" and italic").italic(),
    ])
);
```
*/

pub mod clean;
mod composite;
mod compound;
mod line;
mod line_parser;
mod tbl;
mod inline_template;
mod text_template;
mod text;

pub use {
    composite::{Composite, CompositeStyle},
    compound::{Alignment, Compound},
    line::Line,
    line::MAX_HEADER_DEPTH,
    tbl::{TableRow, TableRule},
    inline_template::InlineTemplate,
    text_template::{SubTemplateExpander, TextTemplate, TextTemplateExpander},
    text::Text,
};

#[macro_use]
extern crate lazy_static;

/// parse a markdown text
pub fn parse_text<'s>(md: &'s str) -> Text<'s> {
    Text::from(md)
}

/// parse a line, which is meant to be part of a markdown text.
/// This function shouldn't usually be used: if you don't want
/// a text you probably need `parse_inline`
pub fn parse_line<'s>(md: &'s str) -> Line<'s> {
    Line::from(md)
}

/// parse a monoline markdown snippet which isn't from a text.
/// Don't produce some types of line: TableRow, Code, ListItem
///  as they only make sense in a multi-line text.
pub fn parse_inline<'s>(md: &'s str) -> Composite<'s> {
    Composite::from_inline(md)
}
