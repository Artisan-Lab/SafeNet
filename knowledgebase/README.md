### 分类
**价值 low high 本身还是针对pattern以及case来说的，与api没有根本关系**
low 表示是完全的直接使用完全没用
high 只要有点用就可以给



| API | Pattern ID | Discription | Pattern Value | Case | 
|---------|---------|---------|---------|---------|
| add | 1 | 本例子是裸指针直接调用add | LOW  | 1-ptr-simple-unsafe-low.rs | 
| align_to | 1 | 把一个数组按位切换类型，目前看来必须unsafe，可以transmute+from_be_bytes，但还是unsafe, 因为这个替换所以给到high | **HIGH**  |1-slice-simple-unsafe-high.rs <br> 1-vec-simple-unsafe-high.rs | 
| align_to_mut | 1 | 同上 | **HIGH**  |1-slice-simple-unsafe-high.rs <br> 2-vec-simple-unsafe-high-high.rs | 
| alloc | 1 | global allocator不可避免unsafe, 但是复杂语义如果尝试使用alloc的话可能有不同形式 | LOW  |1-alloc-simple-unsafe-low.rs <br> 1-alloczero-simple-unsafe-low.rs | 
| Arc::decrement_strong_count | 1 | 太简单 | LOW  |1-arc-count-simple-unsafe.rs | 
| Arc::from_raw | 1 | 太简单 从裸指针构建一个
ARC类型指针 | LOW  |1-arc-count-simple-unsafe.rs | 
| Arc::increment_strong_count | 1 | 太简单 同decrement | LOW  |1-arc-count-simple-unsafe.rs | 
| as_bytes_mut | 1 | 例子是可以把string变成bytes数组来修改值，safe版本是先变成可变字符数组，在目的是修改字符的语义下有意义 | **HIGH** |1-arc-count-simple-unsafe.rs | 
| as_mut | 1 | 太简单到原子 | LOW  | 1-ptr-nn-unsafe-low.rs <br>1-ptr-simple-unsafe-low.rs| 
| as_mut_vec | 1 | 返回string的一个可变引用再返回，有现实语义 并且可以转换 实现这个简单的功能 直觉上有很多种 | **HIGH**  |1-string-simple-unsafe-high.rs | 
| as_ref | 1 | 都是原子操作，case中有3份存疑，它们不是unsafe | LOW  |1-ptr-mut-unsafe-low.rs <br> 1-ptr-nn-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs <br> 1-ptr-unchecked-unsafe-low.rs <br> 1-ptr-uncheckedmut-unsafe-low.rs <br>Q | 
| as_uninit_ref | 1 | 同上 | LOW  |1-ptr-simple-unsafe-low.rs <br> 1-ptr-mut-unsafe-low.rs | 
| assume_init | 1 | 必须确信初始化的才能使用, assumeinit简单的使用 | LOW  |1-box-simple-unsafe-low.rs <br> 1-box-slice-unsafe-lowrs <br> 1-rc-simple-unsafe-low.rs <br> 1-rc-slice-unsafe-low.rs <br> 1-arc-simple-unsafe-low.rs | 
| assume_init | 2 | maybeuninit 带来的assumeinti的 | **HIGH**  | 2-maybeuninit-refi32-unsafe-high.rs <br> 2-maybeuninit-struct-unsafe-high.rs <br> 2-maybeuninit-vec-unsafe-high.rs <br> 2-maybeuninit-zeroed-unsafe-high.rs | 
| assume_init | 3 | 有具体语义，结合了transmute，| **HIGH**   |3-array-maybeuninitstring-unsafe-high.rs <br> 3-array-maybeuninitvec-unsafe-high.rs | 
| assume_init_mut | 1 | 与assumeinit类似 这些实际只有一种 | **HIGH**   |1-assumeinitmut-simple-unsafe-high.rs <br> 1-misused-assumeinitmut-readuninit-unsafe-high.rs <br>  1-misused-assumeinitmut-uninit-unsafe-high.rs <br> 1-misused-assumeinitmut-uninitfield-unsafe-high.rs| 
| assume_init_read | 1 | 与assumeinit类似，这些其实只有一种 | **HIGH**   |1-misused-assumeinitmut-read.rs <br> uninit-unsafe-high.rs <br>  1-assumeinitreadnone-simple-unsafe-high.rs <br>  1-misused-assumeinitread-simple-unsafe-high.rs| 
| assume_init_ref | 1 | 后面两个是误用但是从unsafe转safe角度上是一样的 | **HIGH**   |1-assumeinitref-simple-unsafe.rs <br> 1-misused-assumeinitref-simple-unsafe.rs <br> 1-misused-assumeinitrefcell-simple-unsafe.rs  | 
| Box::from_raw | 1 | 把裸指针装到一个box里进行drop，目的是drop指针指向的东西 无法用safe替换  | **HIGH**   |1-drop-unsafe-2-high.rs <br> 1-drop-unsafe-high.rs | 
| Box::from_raw | 2 | 为了用fromraw而用,先alloc再把位置赋值，转成box传出 case里其实是两个一模一样的代码 | LOW  |2-alloc-unsafe-2-low.rs <br> 2-alloc-unsafe-low.rs| 
| Box::from_raw | 3 | 为了用fromraw而用,用into_raw定义一个裸指针,再转回去 | LOW  |3-simple-unsafe-low.rs <br>3-string-unsafe-2-low.rs| 
| Box::from_raw_in | 1 | 与Box::from_raw pattern2 一样 | LOW  |1-simple-unsafe-low.rs| 
| CStr::from_ptr | 1 | ffi 只单纯调用了一下api | LOW  |1-cstrfromptr-simple-unsafe-low.rs| 
| CStr::from_ptr | 2 | 没看懂意思，先给high | **HIGH**  |2-cstrfromptrconst-simple-unsafe-high.rs| 
| CString::from_raw | 1 | 没看懂意思，先给high | **HIGH**   |1-CStringfromraw-simple-unsafe-high.rs| 
| dealloc| 1 | 存在safe版本, 但是在case中表现的其实是 我分配一个东西再释放掉，太简单，但是不是完全没意义 | **HIGH**   |1-dealloc-simple-unsafe-high.rs <br> 1-deallocfromzero-simple-unsafe-high.rs|
| drop_in_place| 1 | 与dealloc 类似 ，用先分配再释放的方式构造的问题，换了个写法 | **HIGH**   |1-ptr-rc-unsafe-high.rs |
| MaybeUninit::array_assume_init| 1 | 和assumeinit中pattern2出现的实际相同 | **HIGH**   |1-array-simple-unsafe-high.rs|
| mem::align_of_val_raw | 1 | 纯粹调用 | LOW   |1-alignof-simple-unsafe-low.rs|
| mem::size_of_val | 1 | 纯粹调用 | LOW   |1-sizeof-simple-unsafe-low.rs|
| mem::zeroed | 1 | 纯粹调用 | LOW   |1-misused-zeroed-simple-unsafe-low.rs <br> 1-zeroed-simple-unsafe-low.rs|
| offset | 1 | 2个mut 2个const完全没有区别，其实只是调用了下offset展示效果 | LOW   |1-ptr-const-unsafe-1-low.rs <br> 1-ptr-const-unsafe-2-low.rs <br> 1-ptr-mut-unsafe-1-low.rs <br> 1-ptr-mut-unsafe-2-low.rs |
| offset_from | 1 | misused 与普通版其实没区别，都是基础使用 | low | 1-misused-ptr-mut-unsafe-low.rs <br> 1-ptr-mut-unsafe-low.rs <br>1-ptr-simple-unsafe-low.rs |
| Rc::decrement_strong_count| 1 | 基础使用 | low | 1-rc-count-simple-unsafe-low.rs |
| Rc::from_raw | 1 | 与boxfromraw类似，本例与boxfromraw pattern3一致 | low | 1-rcfromraw-simple-unsafe-low.rs |
| Rc::increment_strong_count | 1 | 基础使用 | low | 1-rc+count-simple-unsafe-low.rs |
| read | 1 | 基础使用,其中 所有权与这个例子无关，目的是指针位置读取，所以这两个是同一个pattern | low | 1-ptr-ownership-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs |
| read | 2 | 通过read来进行变量交换，memswap是一个避开所有权冲突的使用方法，所以存在何时使用memswap的问题，这个一定是high | **HIGH** | 2-ptr-read2swap-unsafe-high.rs |
| replace | 1 | 基础使用 | low | 1-ptr-simple-unsafe-low.rs |
| set_len | 1 | 用setlen创建一个vec给ffi用，无法替换 | **HIGH** | 1-vec-ffi-unsafe-high.rs |
| set_len | 2 | 基础使用，但是这个有使用目的，vec的capacity是确定的，但是vec是翻倍增长的，这个例子用来缩掉vec不用的长度，resize | **HIGH** | 2-vec-shorten-unsafe-high.rs |
| slice::from_ptr_range | 1 | 返回两个指针夹着的slice 看std中的描述应该是无法替换,这个case确实只是调了一下，但是这个api确实都可以通过指指针一位一位读来解决，标记为high | **HIGH** | 1-slicefromrange-mut-unsafe-high.rs <br> 1-slicefromrange-simple-unsafe-low.rs|
| sub | 1 | 跟add一样 | LOW  | 1-ptr-simple-unsafe-low.rs <br> 2-ptr-mut-unsafe-low.rs| 
| sub_ptr | 1 | 基础调用，两个case没区别 | LOW  | 1-ptr-mut-unsafe-low.rs <br> 1-ptr-simple-unsafe-low.rs| 
| swap | 1 | swap是一个有复杂语义的api，肯定是high，这里三个case虽然调用api的不同,但是用法是完全一样的，在修改方法上没有不同 | **HIGH**  | 1-ptr-nonoverlapping-unsafe-2-high.rs <br> 1-ptr-nonoverlapping-unsafe-high.rs <br> 1-ptr-overlapping-unsafe-high.rs| 
| Weak::from_raw | 1 | 写的多，但是跟Boxfromraw pattern2 是一样的 | LOW  | 1-weakfromraw-simple-unsafe-low.rs |
| write | 1 | 基础使用 | LOW  | 1-ptr-simple-unsafe.rs |
| write | 2 | swap | **HIGH**  | 2-ptr-write2swap-unsafe-high.rs |

| transmute | 1 | 可以直接用as的用了transmute | **HIGH**   | 1-misused-i32ptrusize-unsafe-high.rs <br> 1-misused-void-unsafe-high.rs|
| transmute | 2 | 把数组转换成数字读取 | **HIGH**   | 2-misused-bytes2u32-unsafe-high.rs |
| transmute | 3 | 数组转数组，有一个结合assumeinit的例子是转数组，可以用相同的方法，所以同pattern | **HIGH**   | 3-vec-option-unsafe-high.rs <br> 3-array-maybeuninit-unsafe.rs |
| transmute | 4 | 裸指针转借用，没法改safe | **HIGH**   | 4-irreplaceable-raw2own-unsafe-high.rs |
| transmute | 5 | 用transmute来修改生命周期，没法改safe | **HIGH**   | 5-lifetime-extend-unsafe.rs <br> 5-lifetime-shrink-unsafe.rs|
| transmute | 6 | 误用，按位读取，想用transmute，价值相较于transmute其他低，同pattern1差不多，因为都是更复杂的误用方式 | **HIGH**   | 6-misused-str2u8-unsafe-high.rs |

| String::from_raw_parts | 1 | from_raw_parts 的基本解决方案，从指针处按位读，读到想要的位置，如果这个指针是safe的，那就能safe。这三个case本质是一样，只是分别是iteminc按位指针处+1，fromvec按位转换类型，1-fromraw-unsafe-high.rs 是一个无目的例子，建议去掉 | **HIGH**  | 1-fromraw-unsafe-high.rs <br> 1-fromvec-unsafe-high.rs <br> 1-iteminc-unsafe-high.rs |
| String::from_raw_parts | 2 | 这个本质上是和 pattern1相同，属于类型转换，但是因为String有自己的to_string方法非常方便，所以改法不同了 | **HIGH**  | 2-fromstr-unsafe-high.rs |
| String::from_raw_parts | 3 | 怕直接用string消耗所有权，所以先用个ptr，用from_raw_parts单纯来生产个ptr，恐怕没有人这么做，价值小，因为是更复杂的用法实现了一个简单的功能 与transmute的as价值差不多 | **HIGH**  | 3-ownership-unsafe.rs |

| Vec::from_raw_parts | 1 | 与String::from_raw_parts问题相同，这些case改法只有一个就是按位读，这里一个case 1-frommem-unsafe-high.rs，看似多，但是其实就是一堆无效操作，就存了一个数。 这里的1-fromraw-unsafe-high.rs也是无目的一个例子，建议去掉| **HIGH**  | 1-frommem-unsafe-high.rs <br> 1-fromraw-unsafe-high.rs <br> 1-iteminc-unsafe-high.rs |

| Vec::from_raw_parts_in | 1 | 这个api和上面的Vec::from_raw_parts问题一模一样 | **HIGH**  | 1-frommem-unsafe-high.rs <br> 1-iteminc-unsafe-high.rs |


