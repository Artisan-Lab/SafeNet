pub fn shift(&mut self) {
    if self.position > 0 {
        unsafe {
            let length = self.end - self.position;
            ptr::copy(
                self.memory[self.position..self.end].as_ptr(),
                self.memory[..length].as_mut_ptr(),
                length,
            );
            self.position = 0;
            self.end = length;
        }
    }
}
// https://github.com/sozu-proxy/sozu/blob/fc293549814a450e5d20bf682cb8002ce55ee5e7/command/src/buffer/growable.rs#L101