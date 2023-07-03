pub fn set_props<T>(&mut self, props: T) {
    let boxed = Box::into_raw(Box::new(props));
    let data = unsafe { ::std::mem::transmute(boxed) };
    self.props = Some((self.type_id, data));
}
/*
https://github.com/VISCHub/yew/blob/02f476d008280033cc7c0355f15754314394e00a/src/virtual_dom/vcomp.rs#L73
*/