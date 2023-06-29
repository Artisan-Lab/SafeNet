pub struct ChildNodeIteratorMut<'a, T> {
    nodes: Vec<&'a mut Node<T>>,
}
fn get_mut_unchecked(&mut self, id: NodeId) -> &mut T {
    unsafe { &mut self.nodes.get_unchecked_mut(id.0).value }
}
/*
https://github.com/iacore/dioxus/blob/d521da1991719760e271457dfe4f9ddf281afbb3/packages/native-core/src/tree.rs#L288
 */