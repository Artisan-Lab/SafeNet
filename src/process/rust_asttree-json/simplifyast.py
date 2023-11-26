import re
import os


input_dir_path = '/home/rose/code/x/ast'  
output_dir_path = '/home/rose/code/x/ast'  


for root, dirs, files in os.walk(input_dir_path):
    for input_file in files:
        var_dict = {}
        
        res_dict = {    
        "unsafe": None,
        "Box": None,
        "MaybeUninit": None,
        "Rc": None,
        "Arc": None,
        "Vec": None,
        "slice": None,
        "str": None,
        "map": None,
        "HashMap": None,
        "vec!": None,
        "alloc": None,
        "String": None,
        "array": None,
        "struct": None,
        "UnsafeCell": None,
        "Option": None,
        "Result": None,
        "Some": None,
        "NonZero": None,
        "NonNull": None,
        "Pin": None,
        "Layout": None,
        "mut": None,
        "const": None,
        "as": None,
        "array_assume_init": None,
        "as_bytes_mut": None,
        "as_chunks_unchecked": None,
        "as_chunks_unchecked_mut": None,
        "as_ptr": None,
        "as_mut_ptr": None,
        "as_ref": None,
        "as_uninit_ref": None,
        "assume_init": None,
        "as_mut": None,
        "cell": None,
        "dealloc": None,
        "clone": None,
        "deallocate": None,
        "drop": None,
        "drop_in_place": None,
        "downcast_unchecked": None,
        "expect": None,
        "forget": None,
        "free": None,
        "from": None,
        "from_le_bytes": None,
        "from_raw": None,
        "from_raw_in": None,
        "from_raw_parts": None,
        "from_raw_parts_mut": None,
        "get": None,
        "get_mut_unchecked": None,
        "get_unchecked": None,
        "get_unchecked_mut": None,
        "into": None,
        "into_raw": None,
        "into_raw_with_allocator": None,
        "len": None,
        "libc": None,
        "malloc": None,
        "new_unchecked": None,
        "new_uninit": None,
        "new_uninit_slice": None,
        "new": None,
        "new_in": None,
        "new_zeroed": None,
        "new_zeroed_slice": None,
        "offset": None,
        "offset_from": None,
        "set_len": None,
        "swap_unchecked": None,
        "split_at_unchecked": None,
        "split_at_mut_unchecked": None,
        "slice_unchecked": None,
        "sub": None,
        "transmute": None,
        "to_int_unchecked": None,
        "to_owned": None,
        "uninit": None,
        "uninit_array": None,
        "unwrap_unchecked": None,
        "unwrap_err_unchecked": None,
        "unchecked_mul": None,
        "unchecked_add": None,
        "wrap": None,
        "wrapping_offset": None,
        "wrapping_add": None,
        "wrapping_sub": None,
        "write": None,
        "zeroed": None,
        "main": None,
        "mem": None, 
        "key": None,
        "ptr": None,
        "self": None,
        "str": None,
        "num": None,
    }
    
        if input_file.endswith('.ast'):
            input_file_path = os.path.join(root, input_file)
            with open(input_file_path, 'r') as f1:
                string = f1.read()
                string = re.sub('ident:[ \w]{1,}\([\n ]*([\w]{1,},)[\n ]*\),', 'ident: \g<1>', string, flags=re.S)
                string = re.sub('method:[ \w]{1,}\([\n ]*([\w]{1,},)[\n ]*\),', 'ident: \g<1>', string, flags=re.S)
                string = string.replace('\n', '').replace(' ', '')
                string = string.replace("{", "(").replace("}", ")").replace("[", "(").replace("]", ")")
                string = string.replace('(eq,', '( eq,').replace('(Mut,', '( Mut,').replace('(Dot2,', '( Dot2,').replace('(Colon2,', '( Colon2,')

                result = re.findall(r'([()])|(ident:\w*)|(\w*token\w*:([^N]\w*))|( eq,)|( Mut,)|( Colon2,)', string, flags=re.I)
                result = [''.join(i).strip().strip(',') for i in result]
                result = [i if i in '()' else f'({i})' for i in result]
                output = ''.join(result)

                result = re.findall('\((ident:\w{1,})\)', output)
                filter_res = [i.split(':') for i in result]

                for i in filter_res:
                    if i[-1].lower() not in res_dict:
                        res_dict[i[-1].lower()] = None

                di = {}
                for i in filter_res:
                    if i[-1].lower() not in di:
                        di[i[-1].lower()] = f'var{len(di) + 1}'
                        res_dict[i[-1].lower()] = None

                for k in di:
                    output = output.replace('ident:' + k, 'ident:' + di[k])

                token_find = re.findall(r'(([^\(\):]*token[^\(\):]*):([^\(\):]+))', output)
                for t, p, s in token_find:
                    output = output.replace(t, p)

                ident_find = re.findall(r'(([^\(\):]*ident[^\(\):]*):([^\(\):]+))', output)
                for t, p, s in ident_find:
                    output = output.replace(t, s)

                output = output.replace("()", "")

                def simplify_bracket(src):
                    stack = []
                    splited = re.split(r"([\(\)])", src)
                    for p in [e for e in splited if e]:
                        if p != ")":
                            stack.append(p)
                        else:
                            temp = ""
                            for i, _ in enumerate(range(len(stack))):
                                s_pop = stack.pop(-1)
                                if s_pop == "(":
                                    if i <= 1:
                                        if "(" not in temp:
                                            stack.append(f"({temp})")
                                        else:
                                            stack.append(temp)
                                    else:
                                        stack.append(f'({temp})')
                                    break
                                else:
                                    temp = s_pop + temp
                    return "".join(stack)

                simplify_tree = simplify_bracket(output)

            with open(input_file_path, 'w') as f2:
                f2.write(simplify_tree)





'''
特殊符号
*       (star_token: Star
         op: Deref(Star,),)          
&       (and_token: And,)
<>      ( lt_token: Lt,  gt_token: Gt,)
::      ( Colon2,
          colon2_token: Colon2,)
[]      (bracket_token: Bracket,)
..      (limits: HalfOpen(Dot2,),)

'''
