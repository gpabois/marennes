use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Keyword {
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

    Normal,
    Italic,
    Oblique
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
        }
    }
}

impl Keyword {
    pub fn is_either(&self, values: &[Self]) -> bool {
        values.iter().find(|value| **value == *self).is_some()
    }

    pub fn is_either_func(values: &[Self]) -> impl Fn(&&Keyword) -> bool + '_ {
        |value| values.iter().find(|v| **v == **value).is_some()
    }
}