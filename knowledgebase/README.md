## Knowledge Base of Unsafe Rust APIs

### Unsafe APIs that can be directly replaced: 

| Pattern Name | API | Replacement Strategy | Pattern ID: Discription |
|---------|---------|---------|---------|
| unchecked | \*unchecked\* | all unchecked APIs can be replaced with a safe API by removing unchecked | ignore boundary check |
| - | get_unchecked | get |
| - | ...over 20... |
| wrapping | add |  wrapping_add | ignore arithmatic overflow | |
| - | sub | wrapping_sub | |
| - | byte_add | wrapping_byte_add｜
| - | byte_sub | wrapping_byte_sub |
| - | byte_offset | wrapping_offset | 
| - | offset | wrapping_offset | |
| - | byte_offset | wrapping_byte_offset |

### Unsafe APIs that need machine learning: 

| ID | API | Pattern ID: Discription | Replaceable? | Practical? | Case | 
|---------|---------|---------|---------|---------|---------|
| 1* | Arc::from_raw | 1: Straightforward: replaceable with Arc::new | Y | M | 1-box2rc-unsafe.rs, 1-vec2rc-unsafe.rs, 1-String2rc-unsafe.rs | 
| - | Arc::from_raw | 2: Raw ptr parameter: irreplaceable | N | Y | 2-rawptr-unsafe.rs | 
| - | Arc::from_raw | 3: Create Arc\<B\> from A: replaceable, Convert A to B first | Y | Y | 3-coersion-unsafe.rs | 
| - | Arc::from_raw | 4: &self parameter: depends on Copy or Clone | M | Y | 4-selfclone2rc-unsafe.rs, 4-selfclone2rc-unsafe.rs|
| 2(new) | Box::from_raw | 1: Raw ptr parameter | N | Y | new1-fromraw-unsafe.rs|
| -  | Box::from_raw | 2: Returned raw ptr | N | Y | new2-dropraw-unsafe.rs |
| -  | Box::from_raw | 3: to other Rust objects | Y | Y | new3-tostr-unsafe.rs,new3-tovec-unsafe.rs |
| -  | Box::from_raw | 4: Ownership issue: use as_ptr instead | Y | Y | new4-coersion-unsafe.rs,new4-copycoersion-unsafe.rs |
| -  | Box::from_raw | 5: Modify Box contents | Y | Y | new5-modify-unsafe.rs |
| -  | Box::from_raw | 6: Function parameters: raw | Y | Y | new6-funparraw-unsafe.rs |
| -  | Box::from_raw | 7: Function parameters: self | M | Y | new7-dropself-unsafe.rs |
| 2* | Box::from_raw | 1: Raw ptr parameter: drop raw pointers | N | Y  | 1-dropraw-unsafe.rs | 
| - | Box::from_raw | 2: Alloc layout<T>: replace with Box::new(T)  | M | Y | 2-alloci32-unsafe.rs |
| - | Box::from_raw | 3: drop(&self) | M | Y | 3-dropself-unsafe.rs |
| - | Box::from_raw | 4: Create Box\<A\> from Box\<B\>: depends on whether casting A to B is allowed | M | Y | 4-coersioncopy-unsafe.rs, 4-coersion-unsafe.rs |
| 3* | Box::from_raw_in | 1: create a Box with a specific allocator | Y | Y | 1-simple-unsafe.rs| 
| 4* | CStr::from_ptr | 1: From an FFI returned raw ptr | N | Y | 1-cstrfromffi-unsafe.rs | 
| - | CStr::from_ptr | 2: Create a new CStr | Y | Y | 2-createcstr-unsafe.rs| 
| - | CStr::from_ptr | 3: Raw ptr parameter | N | Y | 3-cstrfromptr-unsafe.rs | 
| 5(new) | CString::from_raw | 1: Raw ptr parameter | N | Y | new1-fromffi-unsafe.rs,new1-fromraw-unsafe.rs|
| -  | CString::from_raw | 2: Returned raw ptr | N | Y |  |
| -  | CString::from_raw | 3: to other Rust objects | Y | Y |  |
| -  | CString::from_raw | 4: Ownership issue: Retake the ownership after FFI | N | Y | new4-retakeffi-unsafe.rs |
| -  | CString::from_raw | 5: Modify CString contents | Y | Y | new5-modify-unsafe.rs |
| -  | CString::from_raw | 6: Function parameters: raw | Y | Y |  |
| -  | CString::from_raw | 7: Function parameters: self | M | Y |  |
| 5* | CString::from_raw | 1: Retake the ownership after FFI | N | Y | 1-retakeffi-unsafe.rs | 
| - | CString::from_raw | 2: Modify through raw ptr, use as_bytes instead | Y | Y | 2-modify-unsafe.rs| 
| - | CString::from_raw | 3: Raw ptr parameter (drop the content) | N | Y | 3-fromraw-unsafe.rs| 
| 6 | MaybeUninit::array_assume_init| 1:和assumeinit中pattern2出现的实际相同 | **HIGH**   |1-array-simple-unsafe-high.rs|
| 7* | Rc::from_raw | 1: Straightforward: replaceable with Rc::new | Y | M | 1-box2rc-unsafe.rs, 1-vec2rc-unsafe.rs, 1-String2rc-unsafe.rs | 
| - | Rc::from_raw | 2: Raw ptr parameter: irreplaceable | N | Y | 2-rawptr-unsafe.rs | 
| - | Rc::from_raw | 3: Create Rc\<B\> from A: replaceable, Convert A to B first | Y | Y | 3-coersion-unsafe.rs | 
| - | Rc::from_raw | 4: &self parameter: depends on Copy or Clone | M | Y | 4-selfclone2rc-unsafe.rs, 4-selfclone2rc-unsafe.rs|
| 8 | String::from_raw_parts | 1: Raw ptr parameter | N | Y | 1-fromraw-unsafe.rs |
| -  | String::from_raw_parts | 2: Returned raw ptr | N | Y | 2-fromres-unsafe.rs |
| -  | String::from_raw_parts | 3: from other Rust objects | Y | Y | 3-fromstr-unsafe.rs, 3-fromvec-unsafe.rs, 3-fromvec-unsafe2.rs |
| -  | String::from_raw_parts | 4: Ownership issue: use as_ptr instead | Y | Y | 4-ownership-unsafe.rs |
| -  | String::from_raw_parts | 5: Modify String contents | Y | Y | 5-modify-unsafe.rs |
| 9 | Vec::from_raw_parts | 1: Raw ptr parameter | N | Y | 1-fromraw-unsafe.rs |
| -  | Vec::from_raw_parts | 2: Returned raw ptr | N | Y | 2-fromres-unsafe.rs |
| -  | Vec::from_raw_parts | 3: from other Rust objects | Y | Y | 3-frommem-unsafe.rs |
| -  | Vec::from_raw_parts | 4: Ownership issue: use as_ptr instead | Y | Y | 4-ownership-unsafe.rs |
| -  | Vec::from_raw_parts | 5: Modify Vec contents | Y | Y | 5-iteminc-unsafe.rs|
| - | Vec::from_raw_parts_in | 2: 这个api和上面的Vec::from_raw_parts问题一模一样 | **HIGH**  | 1-frommem-unsafe-high.rs <br> 1-iteminc-unsafe-high.rs |
| 10 | Weak::from_raw | 1: 写的多，但是跟Boxfromraw pattern2 是一样的 | LOW  | 1-weakfromraw-simple-unsafe-low.rs |
| 11* | assume_init | 1: Box/Rc/Arc simple use,replaceable, init(new) | Y | N  | 1-box-simple-unsafe-low.rs,1-box-slice-unsafe-lowrs ,1-rc-simple-unsafe-low.rs, 1-rc-slice-unsafe-low.rs,1-arc-simple-unsafe-low.rs | 
| - | assume_init | 2: maybeuninit  | M | Y | 2-maybeuninit-refi32-unsafe-high.rs,2-maybeuninit-struct-unsafe-high.rs,2-maybeuninit-vec-unsafe-high.rs,2-maybeuninit-zeroed-unsafe-high.rs |
| - | assume_init | 3:\[MaybeUninit\]+ Other unsafe APIs | M | Y | 3-array-maybeuninitstring-unsafe-high.rs,3-array-maybeuninitvec-unsafe-high.rs |
| 12* | transmute | 1:misuse,replace with as | Y | Y  | 1-misused-i32ptrusize-unsafe-high-1.rs,1-misused-void-unsafe-high-2.rs | 
| - | transmute | 2: misuse,byte operations，replace with safe APIs:from_le_bytes...  | Y | Y | 2-misused-bytes2u32-unsafe-high-1.rs |
| - | transmute | 3:vec，"into_iter" method converts to an iterator | Y | Y | 3-array-maybeuninit-unsafe-1.rs,3-vec-option-unsafe-high-2.rs,3-vec-string-unsafe-high-3.rs|
| - | transmute | 4: reborrow  | N | Y | 4-irreplaceable-i2u32-unsafe-high-1.rs，4-irreplaceable-raw2own-unsafe-high-2.rs |
| - | transmute | 5:modify lifetimes | M | Y | 5-lifetime-extend-unsafe-1.rs，5-lifetime-shrink-unsafe-2.rs|
| - | transmute | 6:bitwise reading，replace with safe methods using slice and string | Y | Y | 6-misused-str2slice-unsafe-high-1.rs|
| 13* | swap | 1:use slice | Y | Y  | 1-ptr-nonoverlapping-unsafe-high-1.rs,1-ptr-nonoverlapping-unsafe-high-2.rs,1-ptr-overlapping-unsafe-high-3.rs | 
| - | swap | 2: misuse,mem::swap | Y | Y | 2-mem-misuse-unsafe.rs |
| 14* | align_to | 1: array bitwise toggle，another unsafe method（transmute + from_be_bytes）| M | Y  |1-slice-simple-unsafe-high.rs,1-vec-simple-unsafe-high.rs | 
| 15* | align_to_mut | 1: array bitwise toggle，another unsafe method（transmute + from_be_bytes） | M | Y |1-slice-simple-unsafe-high.rs,2-vec-simple-unsafe-high-high.rs | 
| 16 | alloc | 1: global allocator不可避免unsafe, 但是复杂语义如果尝试使用alloc的话可能有不同形式 | LOW  |1-alloc-simple-unsafe-low.rs <br> 1-alloczero-simple-unsafe-low.rs | 
| 17 | as_bytes_mut | 1: 例子是可以把string变成bytes数组来修改值，safe版本是先变成可变字符数组，在目的是修改字符的语义下有意义 | **HIGH** |1-arc-count-simple-unsafe.rs | 
| 18 | as_mut | 1: 太简单到原子 | LOW  | 1-ptr-nn-unsafe-low.rs <br>1-ptr-simple-unsafe-low.rs| 
| 19 | as_mut_vec | 1: 返回string的一个可变引用再返回，有现实语义 并且可以转换 实现这个简单的功能 直觉上有很多种 | **HIGH**  |1-string-simple-unsafe-high.rs | 
| *20 | as_ref | 1: 都是原子操作，case中有3份存疑，它们不是unsafe | LOW  |1-ptr-mut-unsafe-low.rs <br> 1-ptr-nn-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs <br> 1-ptr-unchecked-unsafe-low.rs <br> 1-ptr-uncheckedmut-unsafe-low.rs <br>Q | 
| 21 | as_uninit_mut | 
| 22 | as_uninit_ref | 1: 同上 | LOW  |1-ptr-simple-unsafe-low.rs <br> 1-ptr-mut-unsafe-low.rs | 
| | as_uninit_slice |
| | as_uninit_slice_mut |
| 23 | assume_init_mut | 1: 与assumeinit类似 这些实际只有一种 | **HIGH**   |1-assumeinitmut-simple-unsafe-high.rs <br> 1-misused-assumeinitmut-readuninit-unsafe-high.rs <br>  1-misused-assumeinitmut-uninit-unsafe-high.rs <br> 1-misused-assumeinitmut-uninitfield-unsafe-high.rs| 
| 24 | assume_init_read | 1: 与assumeinit类似，这些其实只有一种 | **HIGH**   |1-misused-assumeinitmut-read.rs <br> uninit-unsafe-high.rs <br>  1-assumeinitreadnone-simple-unsafe-high.rs <br>  1-misused-assumeinitread-simple-unsafe-high.rs| 
| 25 | assume_init_ref | 1: 后面两个是误用但是从unsafe转safe角度上是一样的 | **HIGH**   |1-assumeinitref-simple-unsafe.rs <br> 1-misused-assumeinitref-simple-unsafe.rs <br> 1-misused-assumeinitrefcell-simple-unsafe.rs  | 
| 26 | byte_offset_from? |
| 27 | dealloc| 1: 存在safe版本, 但是在case中表现的其实是 我分配一个东西再释放掉，太简单，但是不是完全没意义 | **HIGH**   |1-dealloc-simple-unsafe-high.rs <br> 1-deallocfromzero-simple-unsafe-high.rs|
| 28 | drop_in_place| 1: 与dealloc 类似 ，用先分配再释放的方式构造的问题，换了个写法 | **HIGH**   |1-ptr-rc-unsafe-high.rs |
| 29 | mem::align_of_val_raw | 1: 纯粹调用 | LOW   |1-alignof-simple-unsafe-low.rs|
| 30 | mem::size_of_val | 1: 纯粹调用 | LOW   |1-sizeof-simple-unsafe-low.rs|
| 31 | mem::zeroed | 1: 纯粹调用 | LOW   |1-misused-zeroed-simple-unsafe-low.rs <br> 1-zeroed-simple-unsafe-low.rs|
| - | ptr::copy |
| - | ptr::copy_nonoverlapping |
| - | ptr::drop_in_place |
| 32 | read | 1: 基础使用,其中 所有权与这个例子无关，目的是指针位置读取，所以这两个是同一个pattern | low | 1-ptr-ownership-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs | 
| - | ptr::read | 2: 通过read来进行变量交换，memswap是一个避开所有权冲突的使用方法，所以存在何时使用memswap的问题，这个一定是high | **HIGH** | 2-ptr-read2swap-unsafe-high.rs |
| - | ptr::read_unaligned |
| - | ptr::replace | 1: 基础使用 | low | 1-ptr-simple-unsafe-low.rs |
| - | ptr::swap | 1: swap是一个有复杂语义的api，肯定是high，这里三个case虽然调用api的不同,但是用法是完全一样的，在修改方法上没有不同 | **HIGH**  | 1-ptr-nonoverlapping-unsafe-2-high.rs <br> 1-ptr-nonoverlapping-unsafe-high.rs <br> 1-ptr-overlapping-unsafe-high.rs| 
| - | ptr::swap_nonoverlapping |
| - | ptr::write | 1: 基础使用 | LOW  | 1-ptr-simple-unsafe.rs |
| - | ptr::write | 2: swap | **HIGH**  | 2-ptr-write2swap-unsafe-high.rs |
| - | ptr::write_bytes | 
| - | ptr::write_unaligned |
| *33 | set_len | 1: 用setlen创建一个vec给ffi用，无法替换 | **HIGH** | 1-vec-ffi-unsafe-high.rs |
| -  |set_len | 2: 基础使用，但是这个有使用目的，vec的capacity是确定的，但是vec是翻倍增长的，这个例子用来缩掉vec不用的长度，resize | **HIGH** | 2-vec-shorten-unsafe-high.rs |
| 34 | slice::from_ptr_range | 1: 返回两个指针夹着的slice 看std中的描述应该是无法替换,这个case确实只是调了一下，但是这个api确实都可以通过指指针一位一位读来解决，标记为high | **HIGH** | 1-slicefromrange-mut-unsafe-high.rs <br> 1-slicefromrange-simple-unsafe-low.rs|
| - | split_at_mut |
| 35 | sub_ptr | 1: 基础调用，两个case没区别 | LOW  | 1-ptr-mut-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs| 
| 36 | offset_from | 1: misused 与普通版其实没区别，都是基础使用 | low | 1-misused-ptr-mut-unsafe-low.rs <br> 1-ptr-mut-unsafe-low.rs <br>1-ptr-simple-unsafe-low.rs |


### Unsafe APIs that can hardly be replaced: 

| Pattern Name | API | Justification |
|---------|---------|---------|
| volatile | read_volatile | we currently don't know other safe APIs that enforce the volatile feature |
| - | ptr::read_volatile | same as read_volatile |
| Rc count | Rc::decrement_strong_count | |
| - | Rc::increment_strong_count | |
| - | Arc::decrement_strong_count | |
| - | Arc::increment_strong_count | |
