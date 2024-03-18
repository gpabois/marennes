/// A box tree fragment
pub enum Fragment {
    /// The fragment is directly the primordial box.
    Box(BoxFragment)
}

pub struct BoxFragment {
    box_id: r#box::BoxTreeNodeId
}

pub struct FragmentTree {

}