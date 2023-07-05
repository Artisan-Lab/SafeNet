/*
https://github.com/tensorflow/rust/blob/c376ddec4ad913f2f50a54a2a5143664fba4704f/src/graph.rs#L518
*/

pub fn import_graph_def_with_return_outputs(
        &mut self,
        graph_def: &[u8],
        options: &ImportGraphDefOptions,
    ) -> Result<Vec<Output>> {
        let buf = Buffer::from(graph_def);
        let mut status = Status::new();
        let n = options.num_return_outputs();
        let mut c_return_outputs: Vec<MaybeUninit<tf::TF_Output>> = Vec::with_capacity(n);
        unsafe {
            c_return_outputs.set_len(n);
            tf::TF_GraphImportGraphDefWithReturnOutputs(
                self.gimpl.inner,
                buf.inner(),
                options.inner,
                c_return_outputs.as_mut_ptr() as *mut tf::TF_Output,
                n as c_int,
                status.inner(),
            );
            status.into_result()?;
            Ok(c_return_outputs
                .iter()
                .map(|x| Output::from_c(self, &x.assume_init()))
                .collect())
        }
    }