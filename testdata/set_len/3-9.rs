/*
https://github.com/blackbeam/mysql_async/blob/ddee16e617525003bc39d49e267e879b8b66e9bc/src/buffer_pool.rs#L62
*/
fn put(self: &Arc<Self>, mut buf: Vec<u8>) {
        // SAFETY:
        // 1. OK – 0 is always within capacity
        // 2. OK - nothing to initialize
        unsafe { buf.set_len(0) }

        buf.shrink_to(self.buffer_size_cap);

        // ArrayQueue will make sure to drop the buffer if capacity is exceeded
        let _ = self.pool.push(buf);
    }