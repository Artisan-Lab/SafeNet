/*
    From: https://github.com/kitech/qt.rs/blob/f2b7331a7296019d0deabd1108f217ea796465d8/src/core/qmetatype.rs#L757
*/

impl<'a> /*trait*/ QMetaType_typeName_s<String> for (i32) {
    fn typeName_s(self ) -> String {
      // let qthis: *mut c_void = unsafe{calloc(1, 80)};
      // unsafe{_ZN9QMetaType8typeNameEi()};
      let arg0 = self  as c_int;
      let mut ret = unsafe {C_ZN9QMetaType8typeNameEi(arg0)};
      let slen = unsafe {strlen(ret as *const i8)} as usize;
      return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
      // return 1;
    }
  }