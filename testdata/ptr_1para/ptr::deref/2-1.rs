fn base(&self) -> &BaseFlow {
    let ptr: *const Self = self;
    let ptr = ptr as *const BaseFlow;
    unsafe { &*ptr }
}
// https://github.com/tdelacour/servo/blob/18c500ecc8fb2d8ed2ff781d3a7f51a405706af5/components/layout_2020/flow.rs#L29