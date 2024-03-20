#[derive(Clone)]
pub enum Display {
    None,
    /// Contents
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
    InlineTable,
}

impl Default for Display {
    fn default() -> Self {
        Self::Block
    }
}
/// The outer display type, which dictates how the principal box itself participates in flow layout.
/// [https://www.w3.org/TR/css-display-3/#outer-role]
pub enum OuterDisplayType {
    Block,
    Inline,
    RunIn,
}

/// The inner display type, which defines (if it is a non-replaced element)
/// the kind of formatting context it generates, dictating how its descendant boxes are laid out.
/// (The inner display of a replaced element is outside the scope of CSS.)
/// [https://www.w3.org/TR/css-display-3/#inner-role]
pub enum InnerDisplayType {
    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby,
}

impl Display {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_contents(&self) -> bool {
        matches!(self, Self::Contents)
    }

    pub fn inner_type(&self) -> Option<InnerDisplayType> {
        if self.is_flow() {
            Some(InnerDisplayType::Flow)
        } else if self.is_flow_root() {
            Some(InnerDisplayType::FlowRoot)
        } else if self.is_flex() {
            Some(InnerDisplayType::Flex)
        } else if self.is_grid() {
            Some(InnerDisplayType::Grid)
        } else if self.is_ruby() {
            Some(InnerDisplayType::Ruby)
        } else if self.is_table() {
            Some(InnerDisplayType::Table)
        } else {
            None
        }
    }

    pub fn is_flow(&self) -> bool {
        matches!(self, Self::Block)
            || matches!(self, Self::RunIn)
            || matches!(self, Self::Inline)
            || matches!(self, Self::ListItem)
            || matches!(self, Self::InlineListItem)
    }

    pub fn is_flow_root(&self) -> bool {
        matches!(self, Self::FlowRoot) || matches!(self, Self::InlineBlock)
    }

    pub fn is_flex(&self) -> bool {
        matches!(self, Self::Flex) || matches!(self, Self::InlineFlex)
    }

    pub fn is_grid(&self) -> bool {
        matches!(self, Self::Grid) || matches!(self, Self::InlineGrid)
    }

    pub fn is_ruby(&self) -> bool {
        matches!(self, Self::Ruby) || matches!(self, Self::BlockRuby)
    }

    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table) || matches!(self, Self::InlineTable)
    }

    /// Returns the outer display type of the element.
    pub fn outer_type(&self) -> Option<OuterDisplayType> {
        if self.is_block() {
            Some(OuterDisplayType::Block)
        } else if self.is_inline() {
            Some(OuterDisplayType::Inline)
        } else if self.is_run_in() {
            Some(OuterDisplayType::RunIn)
        } else {
            None
        }
    }
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block)
            || matches!(self, Self::FlowRoot)
            || matches!(self, Self::ListItem)
            || matches!(self, Self::Flex)
            || matches!(self, Self::Grid)
            || matches!(self, Self::BlockRuby)
            || matches!(self, Self::Table)
    }
    pub fn is_inline(&self) -> bool {
        matches!(self, Self::Inline)
            || matches!(self, Self::InlineBlock)
            || matches!(self, Self::InlineListItem)
            || matches!(self, Self::InlineFlex)
            || matches!(self, Self::Ruby)
            || matches!(self, Self::InlineTable)
    }
    pub fn is_run_in(&self) -> bool {
        matches!(self, Self::RunIn)
    }
}

