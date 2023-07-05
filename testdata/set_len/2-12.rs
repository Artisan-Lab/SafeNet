/*
https://github.com/YutaKanna/the-algorithm/blob/ec83d01dcaebf369444d75ed04b3625a0a645eb9/navi/navi/src/torch_model.rs#L97
*/

 pub fn tensor_to_vec<T: kind::Element>(res: &Tensor) -> Vec<T> {
            let size = TorchModel::tensor_flatten_size(res);
            let mut res_f32: Vec<T> = Vec::with_capacity(size);
            unsafe {
                res_f32.set_len(size);
            }
            res.copy_data(res_f32.as_mut_slice(), size);
            // println!("Copied tensor:{}, {:?}", res_f32.len(), res_f32);
            res_f32
        }