/*
https://github.com/servo/servo/blob/041d95e0f4ef08dddab22e0276204240682cdf79/components/selectors/builder.rs#L143
*/
pub fn build_with_specificity_and_flags(
        &mut self,
        spec: SpecificityAndFlags,
    ) -> ThinArc<SpecificityAndFlags, Component<Impl>> {
        // First, compute the total number of Components we'll need to allocate
        // space for.
        let full_len = self.simple_selectors.len() + self.combinators.len();

        // Create the header.
        let header = HeaderWithLength::new(spec, full_len);

        // Create the Arc using an iterator that drains our buffers.

        // Use a raw pointer to be able to call set_len despite "borrowing" the slice.
        // This is similar to SmallVec::drain, but we use a slice here because
        // we’re gonna traverse it non-linearly.
        let raw_simple_selectors: *const [Component<Impl>] = &*self.simple_selectors;
        unsafe {
            // Panic-safety: if SelectorBuilderIter is not iterated to the end,
            // some simple selectors will safely leak.
            self.simple_selectors.set_len(0)
        }
        let (rest, current) = split_from_end(unsafe { &*raw_simple_selectors }, self.current_len);
        let iter = SelectorBuilderIter {
            current_simple_selectors: current.iter(),
            rest_of_simple_selectors: rest,
            combinators: self.combinators.drain(..).rev(),
        };

        Arc::into_thin(Arc::from_header_and_iter(header, iter))
    }