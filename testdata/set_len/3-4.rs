/*
https://github.com/buckyos/CYFS/blob/e030188895096fd8d91d48753877729f4d37dd24/src/component/cyfs-backup/src/object_pack/aes/writer.rs#L86
*/
async fn add_data(
        &mut self,
        object_id: &ObjectId,
        mut data: Box<dyn AsyncRead + Unpin + Send + Sync + 'static>,
        meta: Option<Vec<u8>>,
    ) -> BuckyResult<BuckyResult<u64>> {
        unsafe {
            self.cache_buf.set_len(0);
        }

        if let Err(e) = data.read_to_end(&mut self.cache_buf).await {
            return Ok(Err(e.into()));
        }

        self.add_cache_data(object_id, meta).await
    }
