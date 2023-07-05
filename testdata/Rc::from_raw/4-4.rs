fn try_emplace<N: TryNew<Output = T>>(n: N) -> Result<Pin<Self>, N::Error> {
    let uninit = Rc::new(MaybeUninit::<T>::uninit());
    unsafe {
      let pinned = Pin::new_unchecked(&mut *(Rc::as_ptr(&uninit) as *mut _));
      n.try_new(pinned)?;
      Ok(Pin::new_unchecked(Rc::from_raw(
        Rc::into_raw(uninit).cast::<T>(),
      )))
    }
  }