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
| 1* | Box::from_raw | 1: Raw ptr parameter | N | Y | new1-fromraw-unsafe.rs|
| -  | Box::from_raw | 2: Returned raw ptr | N | Y | new2-dropraw-unsafe.rs |
| -  | Box::from_raw | 3: to other Rust objects | Y | Y | new3-tostr-unsafe.rs,new3-tovec-unsafe.rs |
| -  | Box::from_raw | 4: Ownership issue: use as_ptr instead | Y | Y | new4-coersion-unsafe.rs,new4-copycoersion-unsafe.rs |
| -  | Box::from_raw | 5: Modify Box contents | Y | Y | new5-modify-unsafe.rs |
| -  | Box::from_raw | 6: Function parameters: raw | Y | Y | new6-funparraw-unsafe.rs |
| -  | Box::from_raw | 7: Function parameters: self | M | Y | new7-dropself-unsafe.rs |
| 2* | CStr::from_ptr | 1: From an FFI returned raw ptr | N | Y | 1-cstrfromffi-unsafe.rs | 
| - | CStr::from_ptr | 2: Create a new CStr | Y | Y | 2-createcstr-unsafe.rs| 
| - | CStr::from_ptr | 3: Raw ptr parameter | N | Y | 3-cstrfromptr-unsafe.rs | 
| 3 | Vec::from_raw_parts | 1: Raw ptr parameter | N | Y | 1-fromraw-unsafe.rs |
| -  | Vec::from_raw_parts | 2: Returned raw ptr | N | Y | 2-fromres-unsafe.rs |
| -  | Vec::from_raw_parts | 3: from other Rust objects | Y | Y | 3-frommem-unsafe.rs |
| -  | Vec::from_raw_parts | 4: Ownership issue: use as_ptr instead | Y | Y | 4-ownership-unsafe.rs |
| -  | Vec::from_raw_parts | 5: Modify Vec contents | Y | Y | 5-iteminc-unsafe.rs|
| - | Vec::from_raw_parts_in | 2: 这个api和上面的Vec::from_raw_parts问题一模一样 | **HIGH**  | 1-frommem-unsafe-high.rs <br> 1-iteminc-unsafe-high.rs |
| 4* | CString::from_raw | 1: Raw ptr parameter | N | Y | new1-fromffi-unsafe.rs,new1-fromraw-unsafe.rs|
| -  | CString::from_raw | 2: Returned raw ptr | N | Y |  |
| -  | CString::from_raw | 3: to other Rust objects | Y | Y |  |
| -  | CString::from_raw | 4: Ownership issue: Retake the ownership after FFI | N | Y | new4-retakeffi-unsafe.rs |
| -  | CString::from_raw | 5: Modify CString contents | Y | Y | new5-modify-unsafe.rs |
| -  | CString::from_raw | 6: Function parameters: raw | Y | Y |  |
| -  | CString::from_raw | 7: Function parameters: self | M | Y |  |
| 5* | Rc::from_raw | 1: Straightforward: replaceable with Rc::new | Y | M | 1-box2rc-unsafe.rs, 1-vec2rc-unsafe.rs, 1-String2rc-unsafe.rs | 
| - | Rc::from_raw | 2: Raw ptr parameter: irreplaceable | N | Y | 2-rawptr-unsafe.rs | 
| - | Rc::from_raw | 3: Create Rc\<B\> from A: replaceable, Convert A to B first | Y | Y | 3-coersion-unsafe.rs | 
| - | Rc::from_raw | 4: &self parameter: depends on Copy or Clone | M | Y | 4-selfclone2rc-unsafe.rs, 4-selfclone2rc-unsafe.rs|
| 6 | mem::uninitialized |1: create and init in the same function | Y | |
| - | mem:: uninitialized | 2: create and init in another function with ref | N | |
| - | mem:: uninitialized | 3: create and init in another function with raw ptr | with MaybeUninit | |
| 7 | mem::zeroed | 1: create and init in the same function | Y | |
| - | mem::zeroed | 2: create and init in another function with ref  | N | |
| - | mem::zeroed | 3: create and init in another function with raw ptr | with MaybeUninit | |
| 8* | assume_init | 1: Create uninit and then init | Y | Y  | 1-box-unsafe.rs, 1-rc-unsafe.rs, 1-arc-unsafe-low.rs, ... | 
| - | assume_init | 2: MaybeUninit parameter  | N | Y |  2-mayi32-unsafe.rs |
| - | assume_init | 3: MaybeUninit retvalue  | N | Y |  3-retmay-unsafe.rs |
| - | assume_init | 4: to be init by other APIs | M | Y |  4-mayvec-unsafe.rs |
| 9* | mem::transmute | 1:misuse,replace with as | Y | Y  | 1-i32ptrusize-unsafe.rs,1-void-unsafe.rs, 1-i2u32-unsafe.rs | 
| - | mem::transmute | 2: misuse, replace with safe APIs:from_le_bytes...  | Y | Y | 2-bytes2u32-unsafe.rs, 2-str2slice-unsafe.rs  |
| - | mem::transmute | 3: convert ContainerA<P> to ContainerA<Q> or ContainerA to ContainerB | Y | Y |  3-vecoption-unsafe-high-2.rs, 3-vecstring-unsafe.rs|
| - | mem::transmute | 4: convert raw to ref  | N | Y | 4-raw2own-unsafe.rs |
| - | mem::transmute | 5: modify lifetimes | M | Y | 5-lifetimeextend-unsafe.rs，5-lifetimeshrink-unsafe.rs |
| - | mem::transmute | 6: Maybeuninit to init | N | Y | 6-maybeuninit-unsafe.rs |
| 10 | ptr::copy |
| 11 | ptr::copy_nonoverlapping |
| 12 | ptr::drop_in_place |
| 13 | ptr::read | 1: 基础使用,其中 所有权与这个例子无关，目的是指针位置读取，所以这两个是同一个pattern | low | 1-ptr-ownership-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs | 
| - | ptr::read | 2: 通过read来进行变量交换，memswap是一个避开所有权冲突的使用方法，所以存在何时使用memswap的问题，这个一定是high | **HIGH** | 2-ptr-read2swap-unsafe-high.rs |
| 14 | ptr::write | 1: 基础使用 | LOW  | 1-ptr-simple-unsafe.rs |
| - | ptr::write | 2: swap | **HIGH**  | 2-ptr-write2swap-unsafe-high.rs |
| 15 | ptr::write_bytes | 
| - | ptr::write_unaligned |
| 16 | set_len | 1: 用setlen创建一个vec给ffi用，无法替换 | **HIGH** | 1-vec-ffi-unsafe-high.rs |
| -  |set_len | 2: 基础使用，但是这个有使用目的，vec的capacity是确定的，但是vec是翻倍增长的，这个例子用来缩掉vec不用的长度，resize | **HIGH** | 2-vec-shorten-unsafe-high.rs |
| 17 | offset_from | 1: misused 与普通版其实没区别，都是基础使用 | low | 1-misused-ptr-mut-unsafe-low.rs <br> 1-ptr-mut-unsafe-low.rs <br>1-ptr-simple-unsafe-low.rs |
| 18 | as_ref | 1: 都是原子操作，case中有3份存疑，它们不是unsafe | LOW  |1-ptr-mut-unsafe-low.rs <br> 1-ptr-nn-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs <br> 1-ptr-unchecked-unsafe-low.rs <br> 1-ptr-uncheckedmut-unsafe-low.rs <br>Q | 
| 19 | * (raw_ptr_deref) | | |  |


### Unsafe APIs that can hardly be replaced: 

| Pattern Name | API | Justification |
|---------|---------|---------|
| volatile | read_volatile | we currently don't know other safe APIs that enforce the volatile feature |
| - | ptr::read_volatile | same as read_volatile |
| Rc count | Rc::decrement_strong_count | |
| - | Rc::increment_strong_count | |
| - | Arc::decrement_strong_count | |
| - | Arc::increment_strong_count | |
