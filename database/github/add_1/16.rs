pub fn get(&self, idx : usize)->Pixel{
    let ptr = self.data.addr as *mut Pixel;
    unsafe {
        *ptr.add(idx)
    }
}