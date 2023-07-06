fn next(&mut self) -> Option<Self::Item> {
    if let Some(simple_selector_ref) = self.current_simple_selectors.next() {
        // Move a simple selector out of this slice iterator.
        // This is safe because we’ve called SmallVec::set_len(0) above,
        // so SmallVec::drop won’t drop this simple selector.
        unsafe { Some(ptr::read(simple_selector_ref)) }
    } else {
        self.combinators.next().map(|(combinator, len)| {
            let (rest, current) = split_from_end(self.rest_of_simple_selectors, len);
            self.rest_of_simple_selectors = rest;
            self.current_simple_selectors = current.iter();
            Component::Combinator(combinator)
        })
    }
}
// https://github.com/revolter-firefox/revolter-firefox/blob/604bdc33b2f92629ee05fc604a82d3cc0182b01d/servo/components/selectors/builder.rs#L177