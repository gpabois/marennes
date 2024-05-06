use std::fmt::Display;

use crate::style::{Style, StyleError};

use super::Value;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Keyword {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,

    Local,
    Scroll,
    Fixed,

    Inherit,
    Unset,
    Initial,
    Revert,
    RevertLayer,

    Block,
    Inline,
    RunIn,

    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby,

    ListItem,

    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,

    None,
    Contents,

    Serif,
    SansSerif,
    Monospace,
    Cursive,
    Fantasy,
    SystemUi,
    UiSerif,
    UiSansSerif,
    UiMonospace,
    UiRounded,
    Emoji,
    Math,
    Fangsong,

    Normal,
    Italic,
    Oblique,
}

impl TryFrom<&str> for Keyword {
    type Error = StyleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "border-box" => Ok(Self::BorderBox),
            "padding-box" => Ok(Self::PaddingBox),
            "content-box" => Ok(Self::ContentBox),
            "text" => Ok(Self::Text),

            "local" => Ok(Self::Local),
            "scroll" => Ok(Self::Scroll),
            "fixed" => Ok(Self::Fixed),

            "inherit" => Ok(Self::Inherit),
            "unset" => Ok(Self::Unset),
            "initial" => Ok(Self::Initial),
            "revert" => Ok(Self::Revert),
            "revert-layer" => Ok(Self::RevertLayer),

            "block" => Ok(Self::Block),
            "inline" => Ok(Self::Inline),
            "run-in" => Ok(Self::RunIn),

            "flow" => Ok(Self::Flow),
            "flow-root" => Ok(Self::FlowRoot),
            "table" => Ok(Self::Table),
            "flex" => Ok(Self::Flex),
            "grid" => Ok(Self::Grid),
            "ruby" => Ok(Self::Ruby),

            "list-item" => Ok(Self::ListItem),

            "table-row-group" => Ok(Self::TableRowGroup),
            "table-header-group" => Ok(Self::TableHeaderGroup),
            "table-footer-group" => Ok(Self::TableFooterGroup),
            "table-row" => Ok(Self::TableRow),
            "table-cell" => Ok(Self::TableCell),
            "table-column-group" => Ok(Self::TableColumnGroup),
            "table-column" => Ok(Self::TableColumn),
            "table-caption" => Ok(Self::TableCaption),
            "ruby-base" => Ok(Self::RubyBase),
            "ruby-text" => Ok(Self::RubyText),
            "ruby-base-container" => Ok(Self::RubyBaseContainer),
            "ruby-text-container" => Ok(Self::RubyTextContainer),
        
            "none" => Ok(Self::None),
            "contents" => Ok(Self::Contents),
        
            "serif" => Ok(Self::Serif),
            "sans-serif" => Ok(Self::SansSerif),
            "monospace" => Ok(Self::Monospace),
            "cursive" => Ok(Self::Cursive),
            "fantasy" => Ok(Self::Fantasy),
            "system-ui" => Ok(Self::SystemUi),
            "ui-serif" => Ok(Self::UiSerif),
            "ui-sans-serif" => Ok(Self::UiSansSerif),
            "ui-monospace" => Ok(Self::UiMonospace),
            "ui-rounded" => Ok(Self::UiRounded),
            "emoji" => Ok(Self::Emoji),
            "math" => Ok(Self::Math),
            "fangsong" => Ok(Self::Fangsong),
        
            "normal" => Ok(Self::Normal),
            "italic" => Ok(Self::Italic),
            "oblique" => Ok(Self::Oblique),

            _ => Err(StyleError::InvalidValue(&["<keyword>"]))
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Inherit => write!(f, "inherit"),
            Keyword::Unset => write!(f, "unset"),
            Keyword::Initial => write!(f, "initial"),
            Keyword::Block => write!(f, "block"),
            Keyword::Inline => write!(f, "inline"),
            Keyword::RunIn => write!(f, "run-in"),
            Keyword::Flow => write!(f, "flow"),
            Keyword::FlowRoot => write!(f, "flow-root"),
            Keyword::Table => write!(f, "table"),
            Keyword::Flex => write!(f, "flex"),
            Keyword::Grid => write!(f, "grid"),
            Keyword::Ruby => write!(f, "ruby"),
            Keyword::ListItem => write!(f, "list-item"),
            Keyword::TableRowGroup => write!(f, "table-row-group"),
            Keyword::TableHeaderGroup => write!(f, "table-header-group"),
            Keyword::TableFooterGroup => write!(f, "table-footer-group"),
            Keyword::TableRow => write!(f, "table-row"),
            Keyword::TableCell => write!(f, "table-cell"),
            Keyword::TableColumnGroup => write!(f, "table-column-group"),
            Keyword::TableColumn => write!(f, "table-column"),
            Keyword::TableCaption => write!(f, "table-caption"),
            Keyword::RubyBase => write!(f, "ruby-base"),
            Keyword::RubyText => write!(f, "ruby-text"),
            Keyword::RubyBaseContainer => write!(f, "ruby-base-container"),
            Keyword::RubyTextContainer => write!(f, "ruby-text-container"),
            Keyword::None => write!(f, "none"),
            Keyword::Contents => write!(f, "contents"),
            Keyword::Normal => write!(f, "normal"),
            Keyword::Italic => write!(f, "italic"),
            Keyword::Oblique => write!(f, "oblique"),
            Keyword::Serif => write!(f, "serif"),
            Keyword::SansSerif => write!(f, "sans-serif"),
            Keyword::Monospace => write!(f, "monospace"),
            Keyword::Cursive => write!(f, "cursive"),
            Keyword::Fantasy => write!(f, "fantasy"),
            Keyword::SystemUi => write!(f, "system-ui"),
            Keyword::UiSerif => write!(f, "ui-serif"),
            Keyword::UiSansSerif => write!(f, "ui-sans-serif"),
            Keyword::UiMonospace => write!(f, "ui-monospace"),
            Keyword::UiRounded => write!(f, "ui-rounded"),
            Keyword::Emoji => write!(f, "emoji"),
            Keyword::Math => write!(f, "math"),
            Keyword::Fangsong => write!(f, "fangsong"),
            Keyword::Local => write!(f, "local"),
            Keyword::Scroll => write!(f, "scroll"),
            Keyword::Fixed => write!(f, "fixed"),
            Keyword::BorderBox => write!(f, "border-box"),
            Keyword::PaddingBox => write!(f, "padding-box"),
            Keyword::ContentBox => write!(f, "content-box"),
            Keyword::Text => write!(f, "text"),
            Keyword::Revert => write!(f, "revert"),
            Keyword::RevertLayer => write!(f, "revert-layer"),
        }
    }
}

impl Keyword {
    pub fn is_either(&self, values: &[Self]) -> bool {
        values.iter().any(|value| *value == *self)
    }

    pub fn is_either_func(values: &[Self]) -> impl Fn(&&Keyword) -> bool + '_ {
        |value| values.iter().any(|v| *v == **value)
    }
}

impl From<Keyword> for Value {
    fn from(value: Keyword) -> Self {
        Self::Keyword(value)
    }
}