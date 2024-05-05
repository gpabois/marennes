use std::fmt::Display;

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
