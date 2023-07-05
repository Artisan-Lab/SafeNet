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
| 1* | Box::from_raw | 1: Function parameters: raw | N |  |
| -  | Box::from_raw | 2: Function parameters: self | M |   |
| -  | Box::from_raw | 3: Returned raw ptr | N |   |
| -  | Box::from_raw | 4: Used by another func | Y |  |
| -  | Box::from_raw | 5: to other Rust objects | Y |   |
| -  | Box::from_raw | 6: Modify Box contents | Y |   |
| 2* | CStr::from_ptr | 1: Function parameters: raw | N |   | 
| -  | CStr::from_ptr | 2: Function parameters: self | M |   |
| -  | CStr::from_ptr | 3: Returned raw ptr | N |  |
| -  | CStr::from_ptr | 4: Used by another func | Y |  |
| -  | CStr::from_ptr | 5: to other Rust objects | Y |  |
| -  | CStr::from_ptr | 6: Modify contents | Y |   |
| 3* | Vec::from_raw_parts | 1: Function parameters: raw | N |   |
| -  | Vec::from_raw_parts | 2: Function parameters: self | M |   |
| -  | Vec::from_raw_parts | 3: Returned raw ptr | N |  |
| -  | Vec::from_raw_parts | 4: Used by another func | Y | |
| -  | Vec::from_raw_parts | 5: to other Rust objects | Y |  |
| -  | Vec::from_raw_parts | 6: Modify Vec contents | Y |  |
| 4* | CString::from_raw | 1: Function parameters: raw | N |    |
| -  | CString::from_raw | 2: Function parameters: self | M |    |
| -  | CString::from_raw | 3: Returned raw ptr | N |    |
| -  | CString::from_raw | 4: Used by another func | Y |    |
| -  | CString::from_raw | 5: to other Rust objects | Y |     |
| -  | CString::from_raw | 6: Modify contents | Y |  |
| 5* | Rc::from_raw | 1: Straightforward: replaceable with Rc::new | Y |     | 
| -  | Rc::from_raw |  Function parameters: raw | M |    |
| -  | Rc::from_raw | 3: Returned raw ptr | N |      |
| -  | Rc::from_raw | 4: Used by another func | Y |     |
| -  | Rc::from_raw | 5: to other Rust objects | Y |      |
| -  | Rc::from_raw | 6: Modify contents | Y |   |
| 6 | mem::uninitialized | 1: The function with Rust primitive types | Y (#derive[Default)| ] | |
| - | mem::uninitialized | 2: create and init in the same function | Y | |
| - | mem:: uninitialized | 3: create and init in another function with ref | N | |
| - | mem:: uninitialized | 4: create and init in another function with raw ptr | with MaybeUninit | |
| 7 | mem::zeroed | 1: The function with Rust primitive types | Y (#derive[Default)| ] | |
| 2 | mem::zeroed | 2: create and init in the same function | Y | |
| - | mem::zeroed | 3: create and init in another function with ref  | N | |
| - | mem::zeroed | 4: create and init in another function with raw ptr | with MaybeUninit | |
| 8* | assume_init | 1: Create uninit and then init | Y | Y  |   | 
| - | assume_init | 2: MaybeUninit parameter  | N | Y |      |
| - | assume_init | 3: MaybeUninit retvalue  | N | Y |      |
| - | assume_init | 4: to be init by other APIs | M | Y |    |
| 8* | mem::transmute | 1:misuse,replace with as | Y | Y  |      | 
| - | mem::transmute | 2: misuse, replace with safe APIs:from_le_bytes...  | Y |  |
| - | mem::transmute | 3: convert ContainerA<P> to ContainerA<Q> or ContainerA to ContainerB | Y |  |
| - | mem::transmute | 4: convert raw to ref  | N | Y |      |
| - | mem::transmute | 5: modify lifetimes | M | Y |     |
| - | mem::transmute | 6: Maybeuninit to init | N | Y |     |
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
| 15 | set_len | 1: reserve space, init via another function, and then set len | May | new or push  |
| - | set_len | 2: reserve space, set len, and then init via another function |  Should | at least pattern 1  |
| -  |set_len | 3: vector shrink: to avoid dual ownership | Hard | pop/remove | 
| ?  |set_len | 4: used in the cleanup function | Hard | pop/remove | 
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
