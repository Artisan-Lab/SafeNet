use std::mem::MaybeUninit;
pub fn into_resource(self) -> Result<Resource, String> {
    let mut array: [MaybeUninit<String>; 20] = unsafe { MaybeUninit::uninit().assume_init() };
    for (level, element) in array.iter_mut().enumerate().map(|(a, b)| (a + 1, b)) {
        let mut ctx = meval::Context::new();
        ctx.var("level", level as f64);
        *element = MaybeUninit::new(format!(
            "{}{}{}",
            self.prefix,
            expr.eval_with_context(ctx).map_err(|x| x.to_string())?,
            self.postfix
        ));
    }
}
/*
https://github.com/cramt/dnd_wiki/blob/643d21cf1730c0566954ded45a65893c274be27e/src/in_model/class/resource.rs#L25
*/
