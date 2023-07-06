pub(crate) unsafe fn from_raw(ptr: *mut GreenTriviaData) -> Self {
    if let Some(ptr) = ptr.as_ref() {
        let arc = Arc::from_raw(&ptr.data as *const ReprThin);
        let arc = mem::transmute::<Arc<ReprThin>, ThinArc<GreenTriviaHead, TriviaPiece>>(arc);
        Self { ptr: Some(arc) }
    } else {
        Self { ptr: None }
    }
}
/*

https://github.com/NikolaRHristov/tools/blob/05ceb80720d36deb1076c3e75ff4d3b83dc32784/crates/rome_rowan/src/green/trivia.rs#L134
*/