pub struct Style {
    pub position: Position,
    pub display: Display
}

pub enum Display {
    None,
    Contents,
    /// block flow
    Block,
    /// block flow-Root
    FlowRoot,
    /// inline flow
    Inline,
    /// inline flow-root
    InlineBlock,
    /// run-in flow
    RunIn,
    /// block flow list-item
    ListItem,
    /// inline flow list-item
    InlineListItem,
    /// block flex
    Flex,
    /// inline flex
    InlineFlex,
    /// grid
    Grid,
    /// inline grid
    InlineGrid,
    /// inline ruby
    Ruby,
    /// block ruby
    BlockRuby,
    /// block table
    Table,
    /// inline table
    InlineTable
}

impl Display {
    pub fn is_block(&self) -> bool {
        match self {
            Self::Block => true,
            Self::FlowRoot => true,
            Self::ListItem => true,
            Self::Flex => true,
            Self::Grid => true,
            Self::BlockRuby => true,
            Self::Table => true,
            _ => false
        }
    }
    pub fn is_inline(&self) -> bool {
        match self {
            Self::Inline => true,
            Self::InlineBlock => true,
            Self::InlineListItem => true,
            Self::InlineFlex => true,
            Self::InlineGrid => true,
            Self::Ruby => true,
            Self::InlineTable => true,
            _ => false
        }

    }
    pub fn is_run_in(&self) -> bool {
        match self {
            Self::RunIn => true,
            _ => false
        }
    }
}

pub enum Position {
    Relative,
    Absolute,
    Static,
    Fixed,
}