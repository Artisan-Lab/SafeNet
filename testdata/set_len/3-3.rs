/*
https://github.com/wasmerio/wasmer/blob/e1edc6e08c34c8fec3bce661e819fd85a8358e4f/lib/virtual-fs/src/cow_file.rs#L209
*/
fn unlink(&mut self) -> BoxFuture<'static, crate::Result<()>> {
        unsafe{let ret = self.buf.set_len(0);}
        Box::pin(async move { ret })
    }