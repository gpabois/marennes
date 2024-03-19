use super::Edge;
use crate::{document, geometry::Block, style};

///
pub struct Box<Unit> {
    /// Containing block of the box
    /// Source: https://www.w3.org/TR/CSS22/visuren.html#containing-block
    pub content: Block<Unit>,
    /// The padding edge surrounds the box’s padding. If the padding has zero width on a given side, the padding edge coincides with the content edge on that side.
    /// The four sides of the padding edge together define the box’s padding box, which contains both the content and padding areas.
    /// Source: https://drafts.csswg.org/css-box-3/#padding-box
    pub padding: Edge<Unit>,
    /// The border edge surrounds the box’s border.
    /// Source: https://drafts.csswg.org/css-box-3/#border-box
    pub border: Edge<Unit>,
    /// The margin edge surrounds the box’s margin
    /// Source: https://drafts.csswg.org/css-box-3/#margin-box
    pub margin: Edge<Unit>,
    /// The element from which the box is generated
    /// Anonymous boxes has no element
    pub element: Option<document::NodeId>,
    /// Style properties
    pub style: style::Style,
}
