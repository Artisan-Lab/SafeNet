pub fn shift(&mut self) {
    let pos = self.inner.position;
    let end = self.inner.end;
    if pos > 0 {
        unsafe {
            let length = end - pos;
            ptr::copy(
                self.inner.extra()[pos..end].as_ptr(),
                self.inner.extra_mut()[..length].as_mut_ptr(),
                length,
            );
            self.inner.position = 0;
            self.inner.end = length;
        }
    }
}
// https://github.com/sozu-proxy/sozu/blob/fc293549814a450e5d20bf682cb8002ce55ee5e7/lib/src/pool.rs#L185