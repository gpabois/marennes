use crate::style::{Value, Keyword};

const ALLOWED_KWS: &'static [Keyword] = &[
    Keyword::Inline,
    Keyword::Block,
    Keyword::Inline,
    Keyword::None,
    Keyword::Contents
];

const BOX_MASK: i32 = 0b11;
const NONE: i32 = 0b10;
const CONTENTS: i32 = 0b11;

const OUTSIDE_MASK: i32 = 0b11_00;
const INLINE: i32 = 0b01_00;
const BLOCK: i32 = 0b10_00;
const RUN_IN: i32 = 0b11_00;

const INSIDE_MASK: i32 = 0b111_00_00;
const FLOW: i32 = 0b001_00_00;
const FLOW_ROOT: i32 = 0b010_00_00;
const TABLE: i32 = 0b011_00_00;
const FLEX: i32 = 0b100_00_00;
const GRID: i32 = 0b101_00_00;
const RUBY: i32 = 0b110_00_00;

const LIST_ITEM_MASK: i32 = 0b1_000_00_00;
const LIST_ITEM: i32 = 0b1_000_00_00;

const INTERNAL_MASK: i32 = 0b1111_0_000_00_00;
const TABLE_ROW_GROUP: i32 = 0b0001_0_000_00_00;
const TABLE_HEADER_GROUP: i32 = 0b0010_0_000_00_00;
const TABLE_FOOTER_GROUP: i32 = 0b0011_0_000_00_00;
const TABLE_ROW: i32 = 0b0100_0_000_00_00;
const TABLE_CELL: i32 = 0b0100_0_000_00_00;
const TABLE_COLUMN_GROUP: i32 = 0b0101_0_000_00_00;
const TABLE_CAPTION: i32 = 0b0110_0_000_00_00;
const RUBY_BASE: i32 = 0b0111_0_000_00_00;
const RUBY_BASE_CONTAINER: i32 = 0b1000_0_000_00_00;
const RUBY_TEXT_CONTAINER: i32 = 0b1001_0_000_00_00;

/// Transform a keyword into a display flag.
fn from_kw(kw: &Keyword) -> i32 {
    match kw {
        Keyword::Block => BLOCK,
        Keyword::Inline => INLINE,
        Keyword::RunIn => RUN_IN,
        
        Keyword::Flow => FLOW,
        Keyword::FlowRoot => FLOW_ROOT,
        Keyword::Table => TABLE,
        Keyword::Flex => FLEX,
        Keyword::Grid => GRID,
        Keyword::Ruby => RUBY,

        Keyword::ListItem => LIST_ITEM,

        Keyword::TableRowGroup => TABLE_ROW_GROUP,
        Keyword::TableHeaderGroup => TABLE_HEADER_GROUP,
        Keyword::TableFooterGroup => TABLE_FOOTER_GROUP,
        Keyword::TableRow => TABLE_ROW,
        Keyword::TableCell => TABLE_CELL,
        Keyword::TableColumnGroup => TABLE_COLUMN_GROUP,
        Keyword::TableCaption => TABLE_CAPTION,
        Keyword::RubyBase => RUBY_BASE,
        Keyword::RubyBaseContainer => RUBY_BASE_CONTAINER,
        Keyword::RubyTextContainer => RUBY_TEXT_CONTAINER,

        Keyword::None => NONE,
        Keyword::Contents => CONTENTS,

        _ => 0
    }
}

// Transform a display flag into a list of keywords.
fn to_kws(flags: i32) -> Vec<Keyword> {
    let box_display = flags & BOX_MASK;
    
    if box_display == NONE {
        return vec![Keyword::None]
    }

    if box_display == CONTENTS {
        return vec![Keyword::Contents]
    }

    let mut kws = Vec::<Keyword>::default();
    
    let inside_display = flags & INSIDE_MASK;
    if inside_display == INLINE {
        kws.push(Keyword::Inline);
    }
    else if inside_display == BLOCK {
        kws.push(Keyword::Block);
    }
    else if inside_display == RUN_IN {
        kws.push(Keyword::RunIn)
    }
    
    let outside_display = flags & OUTSIDE_MASK;
    if outside_display == FLOW {
        kws.push(Keyword::Flow)
    }
    else if outside_display == FLOW_ROOT {
        kws.push(Keyword::FlowRoot)
    }
    else if outside_display == TABLE {
        kws.push(Keyword::Table)
    }
    else if outside_display == FLEX {
        kws.push(Keyword::Flex)
    }
    else if outside_display == GRID {
        kws.push(Keyword::Grid)
    }
    else if outside_display == RUBY {
        kws.push(Keyword::Ruby)
    }

    let list_item_display = flags & LIST_ITEM_MASK;
    if list_item_display == LIST_ITEM {
        kws.push(Keyword::ListItem)
    }

    let internal_display = flags & INTERNAL_MASK;
    if internal_display == TABLE_ROW_GROUP {
        kws.push(Keyword::TableRowGroup);
    }
    else if internal_display == TABLE_HEADER_GROUP {
        kws.push(Keyword::TableHeaderGroup);
    }
    else if internal_display == TABLE_FOOTER_GROUP {
        kws.push(Keyword::TableFooterGroup);
    }
    else if internal_display == TABLE_ROW {
        kws.push(Keyword::TableRow);
    }
    else if internal_display == TABLE_CELL {
        kws.push(Keyword::TableCell)
    }
    else if internal_display == TABLE_COLUMN_GROUP {
        kws.push(Keyword::TableColumnGroup)
    }
    else if internal_display == TABLE_CAPTION {
        kws.push(Keyword::TableCaption)
    }
    else if internal_display == RUBY_BASE {
        kws.push(Keyword::RubyBase)
    }
    else if internal_display == RUBY_BASE_CONTAINER {
        kws.push(Keyword::RubyBaseContainer)
    }
    else if internal_display == RUBY_TEXT_CONTAINER {
        kws.push(Keyword::RubyTextContainer)
    }
    kws
}

/// Correct the display flags.
/// If a box display is set, remove all other flags.
/// If an internal display, remove all other flags.
fn correct_flags(mut flags: i32) -> i32 {
    if flags == 0 {
        flags = INLINE
    }

    if BOX_MASK & flags > 0 {
        flags &= BOX_MASK;
    }

    if INTERNAL_MASK & flags > 0 {
        flags &= INTERNAL_MASK;
    } 

    // The inside values range from 001 to 110
    // > 110 fallbacks to INLINE.
    if INSIDE_MASK & flags > RUBY {
        flags = INLINE;
    }

    // The internal values range from 0001 to 1001.
    // So > 1001 fallbacks to INLINE.
    if INTERNAL_MASK & flags > RUBY_TEXT_CONTAINER {
       flags = INLINE;
    }

    // List item can also have inside display and outside display.
    if LIST_ITEM & flags > 0 {
        flags = (LIST_ITEM | INSIDE_MASK | OUTSIDE_MASK) & flags;

        // > If no inner display type value is specified, 
        // >     the principal box’s inner display type defaults to *flow*. 
        // > If no outer display type value is specified, the 
        // >    principal box’s outer display type defaults to *block*.
        // [CSS Display Module Level 3](https://drafts.csswg.org/css-display/#list-items)
        if flags & INSIDE_MASK == 0 {
            flags |= FLOW;
        }

        if flags & OUTSIDE_MASK == 0 {
            flags |= BLOCK;
        }
    }

    
    if BOX_MASK & flags == 0 || INTERNAL_MASK & flags == 0 {
        if INSIDE_MASK & flags == 0 {
            flags |= FLOW;
        }

        if OUTSIDE_MASK & flags == 0 {
            flags |= INLINE;
        }
    }

    return flags
}

#[derive(Clone, Copy)]
pub struct Display(i32);

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kws = to_kws(self.0)
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        
        write!(f, "display: {}", kws)
    }
}

impl Display {
    fn new(flags: i32) -> Self {
        Self(correct_flags(flags))
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0 & BOX_MASK == NONE
    }

    #[inline]
    pub fn is_contents(&self) -> bool {
        self.0 & BOX_MASK == CONTENTS
    }

}

impl From<Display> for Value {
    fn from(value: Display) -> Self {
        to_kws(value.0).into_iter().collect()
    }
}

impl Default for Display {
    fn default() -> Self {
        Display(INLINE)
    }
}

impl From<Value> for Display {
    fn from(value: Value) -> Self {
        let flags = value
            .iter_keywords()
            .filter(Keyword::is_either_func(&ALLOWED_KWS))
            .map(from_kw)
            .reduce(|a, b| a | b)
            .unwrap_or(INLINE);

        Self::new(flags)
    }
}

#[cfg(test)]
mod tests {
    use super::Display;

    use crate::style::{Keyword, Property};

    #[test]
    fn test_001_display_box() {
        let display = Property::<Display>::new(Keyword::None);
        assert!(display.initial.is_none(), "{};", display.initial)
    }
}