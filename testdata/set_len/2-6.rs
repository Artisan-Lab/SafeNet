/*
https://github.com/lyledean1/tensorflow-rust/blob/ba4fdc72543df508b992c2a2fd8831209b3248b4/src/while_loop.rs#L153
*/
pub fn finish(mut self) -> Result<Vec<Output>> {
        let status = Status::new();
        let mut c_outputs: Vec<tf::TF_Output> =
            Vec::with_capacity(self.inner.inner.ninputs as usize);

        let mut name = None;
        mem::swap(&mut self.name, &mut name);
        let name = match name {
            None => {
                // Include /Merge because while_loop_{} doesn't describe an
                // operation on its own.
                let while_loop_index = self.graph.generate_operation_name("while_loop_{}/Merge")?;
                CString::new(format!("while_loop_{}", while_loop_index))?
            }
            Some(name) => name,
        };
        self.inner.inner.name = name.as_ptr();

        unsafe {
            c_outputs.set_len(self.inner.inner.ninputs as usize);
            for c_output in &mut c_outputs {
                // For some reason, these have to be initialized to {null, -1},
                // even though they're output parameters.
                c_output.oper = ptr::null_mut();
                c_output.index = -1;
            }
            self.inner.finished = true; // used by Drop impl
            tf::TF_FinishWhile(&self.inner.inner, status.inner, c_outputs.as_mut_ptr());
        }
        status.into_result()?;
        Ok(c_outputs
            .iter()
            .map(|out| Output::from_c(self.graph, out))
            .collect())
    }
}