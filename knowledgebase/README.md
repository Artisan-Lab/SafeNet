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

| ID | API | Pattern ID: Discription | Replaceable? | Replacement Strategy | 
|---------|---------|---------|---------|---------|
| 1* | Box::from_raw | 1: Function parameters: raw | N | 1-funparraw-unsafe.rs |
| -  | Box::from_raw | 2: Function parameters: self | M | 2-dropself-unsafe.rs |
| -  | Box::from_raw | 3: Returned raw ptr | N |  3-dropraw-unsafe.rs |
| -  | Box::from_raw | 4: Used by another func | Y | 4-fromraw-unsafe.rs|
| -  | Box::from_raw | 5: to other Rust objects | Y |  5-tostr-unsafe.rs,5-tovec-unsafe.rs |
| -  | Box::from_raw | 6: Modify Box contents | Y | 6-modify-unsafe.rs |
| 2* | CStr::from_ptr | 1: Function parameters: raw | N |  1-funparraw-unsafe.rs,1-funparraw2-unsafe.rs,1-funparraw3-unsafe.rs | 
| -  | CStr::from_ptr | 2: Function parameters: self | M | 2-funparself-unsafe.rs |
| -  | CStr::from_ptr | 3: Returned raw ptr | N |  3-creatcstr-unsafe.rs,3-cstrfromffi-unsafe.rs,3-fromstring-unsafe.rs |
| -  | CStr::from_ptr | 4: Used by another func | Y | 4-resraw-unsafe.rs,4-resraw2-unsafe.rs|
| -  | CStr::from_ptr | 5: to other Rust objects | Y | 5-cstrtobox-unsafe.rs |
| -  | CStr::from_ptr | 6: Modify contents | Y | 6-modify-unsafe.rs |
| 3* | Vec::from_raw_parts | 1: Function parameters: raw | N | 1-funparraw-unsafe.rs |
| -  | Vec::from_raw_parts | 2: Function parameters: self | M |  2-funparself-unsafe.rs |
| -  | Vec::from_raw_parts | 3: Returned raw ptr | N | 3-returnbyother-unsafe.rs |
| -  | Vec::from_raw_parts | 4: Used by another func | Y | 4-usedbyother-unsafe.rs|
| -  | Vec::from_raw_parts | 5: to other Rust objects | Y | 5-frommem-unsafe.rs|
| -  | Vec::from_raw_parts | 6: Modify Vec contents | Y | 6-iteminc-unsafe.rs|
| 4* | CString::from_raw | 1: Function parameters: raw | N |1-parraw-unsafe.rs,1-parraw2-unsafe.rs|
| -  | CString::from_raw | 2: Function parameters: self | M | 2-parself-unsafe.rs  |
| -  | CString::from_raw | 3: Returned raw ptr | N |  3-fromfoo-unsafe.rs |
| -  | CString::from_raw | 4: Used by another func | Y | 4-retakeffi-unsafe.rs|
| -  | CString::from_raw | 5: to other Rust objects | Y | 5-cstrtoboxstr-unsafe.rs,5-vectostr-unsafe.rs  |
| -  | CString::from_raw | 6: Modify contents | Y | 6-modify-unsafe.rs |
| 5* | Rc::from_raw | 1: Straightforward: replaceable with Rc::new | Y | 1-rawptr-unsafe.rs | 
| -  | Rc::from_raw |  Function parameters: raw | M | 2-selfcopy2rc-unsafe.rs,2-selfclone2rc-unsafe.rs |
| -  | Rc::from_raw | 3: Returned raw ptr | N | 3-fromraw-unsafe.rs |
| -  | Rc::from_raw | 4: Used by another func | Y | 4-vec2rc-unsafe.rs,4-box2rc-unsafe.rs,4-String2rc-unsafe.rs|
| -  | Rc::from_raw | 5: to other Rust objects | Y | 5-coersion-unsafe.rs  |
| -  | Rc::from_raw | 6: Modify contents | Y | 6-modify-unsafe.rs |
| 6 | mem::uninitialized | 1: The function with Rust primitive types | Y (#derive[Default)| ] | |
| - | mem::uninitialized | 2: create and init in the same function | Y | |
| - | mem:: uninitialized | 3: create and init in another function with ref | N | |
| - | mem:: uninitialized | 4: create and init in another function with raw ptr | with MaybeUninit | |
| 7 | mem::zeroed | 1: The function with Rust primitive types | Y (#derive[Default)| ] | |
| 2 | mem::zeroed | 2: create and init in the same function | Y | |
| - | mem::zeroed | 3: create and init in another function with ref  | N | |
| - | mem::zeroed | 4: create and init in another function with raw ptr | with MaybeUninit | |
| 8* | assume_init | 1: Create uninit and then init | Y | Y  | 1-box-unsafe.rs, 1-rc-unsafe.rs, 1-arc-unsafe-low.rs, ... | 
| - | assume_init | 2: MaybeUninit parameter  | N | Y |  2-mayi32-unsafe.rs |
| - | assume_init | 3: MaybeUninit retvalue  | N | Y |  3-retmay-unsafe.rs |
| - | assume_init | 4: to be init by other APIs | M | Y |  4-mayvec-unsafe.rs |
| 8* | mem::transmute | 1:misuse,replace with as | Y | Y  | 1-i32ptrusize-unsafe.rs,1-void-unsafe.rs, 1-i2u32-unsafe.rs | 
| - | mem::transmute | 2: misuse, replace with safe APIs:from_le_bytes...  | Y | Y | 2-bytes2u32-unsafe.rs, 2-str2slice-unsafe.rs  |
| - | mem::transmute | 3: convert ContainerA<P> to ContainerA<Q> or ContainerA to ContainerB | Y | Y |  3-vecoption-unsafe-high-2.rs, 3-vecstring-unsafe.rs|
| - | mem::transmute | 4: convert raw to ref  | N | Y | 4-raw2own-unsafe.rs |
| - | mem::transmute | 5: modify lifetimes | M | Y | 5-lifetimeextend-unsafe.rs，5-lifetimeshrink-unsafe.rs |
| - | mem::transmute | 6: Maybeuninit to init | N | Y | 6-maybeuninit-unsafe.rs |
| 9 | ptr::copy | 1: the ref/owner is already available in the current function (misused) | Y  |  | 
| - | ptr::copy | 2: only the raw ptr is available in the current function | N | | 
| 10 | ptr::copy_nonoverlapping || 1: the ref/owner is already available in the current function (misused) | Y  |  | 
| - | ptr::copy_nonoverlapping | 2: only the raw ptr is available in the current function | N | | 
| 11 | ptr::read | 1: the ref/owner is already available in the current function (misused) | Y  |  | 
| - | ptr::read | 2: only the raw ptr is available in the current function | N | | 
| 12 | ptr::write | 1: the ref/owner is already available in the current function (misused) | Y  |  | 
| - | ptr::write | 2: only the raw ptr is available in the current function | N | | 
| 13 | ptr::write_bytes | 1: the ref/owner is already available in the current function (misused) | Y  |  | 
| - | ptr::write_bytes | 2: only the raw ptr is available in the current function | N | | 
| 14 | ptr::drop_in_place |
| 15 | set_len | 1: vector extention: initialize the content and increase the length |  May | new or push  |
| - | set_len | 2: vector extension: increase the length and then initialize the content, similar to mem::uninitialized |  May | new or push or pattern 1  |
| -  |set_len | 3: vector shrink: derease the length and then destruct the content |  May | pop/remove | 
| ?  |set_len | 4: vector shrink: destruct the content and then derease the length |  May | pop/remove | 
| 16 | offset_from | 1: the ref/owner is already available in the current function (misused) | Y  |  | 
| - | offset_from | 2: only the raw ptr is available in the current function | N | | 
| 17 | * (raw_ptr_deref) | 1: the ref/owner is already available in the current function (misused) | Y  |  | 
| - | raw_ptr_deref  | 2: only the raw ptr is available in the current function, and accessing the content is needed | N | | 
| 18 | as_ref | 1: the ref/owner is already available in the current function | Y  |  |
| - | as_ref | 2: only the raw ptr is available in the current function, and the ref is needed | N | | 
| ? | as_ref | 3: only the raw ptr is available in the current function, and the ref is not needed | Y | | 

### Unsafe APIs that can hardly be replaced: 

| Pattern Name | API | Justification |
|---------|---------|---------|
| volatile | read_volatile | we currently don't know other safe APIs that enforce the volatile feature |
| - | ptr::read_volatile | same as read_volatile |
| Rc count | Rc::decrement_strong_count | |
| - | Rc::increment_strong_count | |
| - | Arc::decrement_strong_count | |
| - | Arc::increment_strong_count | |
