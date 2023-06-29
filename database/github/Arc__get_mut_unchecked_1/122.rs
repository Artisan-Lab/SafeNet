pub fn clean(res: &mut Arc<Resource>) {
    unsafe {
        let mut resclone = res.clone();
        let mutres = Arc::get_mut_unchecked(&mut resclone);
        if let Some(ref mut parent) = mutres.parent {
            if Arc::strong_count(res) <= 3 && res.childs.is_empty() {
                log::debug!("Unregister resource {}", res.name());
                for match_ in &mut mutres.matches {
                    let mut match_ = match_.upgrade().unwrap();
                    if !Arc::ptr_eq(&match_, res) {
                        Arc::get_mut_unchecked(&mut match_)
                            .matches
                            .retain(|x| !Arc::ptr_eq(&x.upgrade().unwrap(), res));
                    }
                }
                {
                    Arc::get_mut_unchecked(parent).childs.remove(&res.suffix);
                }
                Resource::clean(parent);
            }
        }
    }
}
/*
https://github.com/KeplerC/zenoh/blob/8b48dd942da59c47fc647a1600652c83cadb64be/zenoh-router/src/routing/resource.rs#L106
*/