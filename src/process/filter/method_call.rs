// 处理一个hash表保存所有method call name self signature  以及 return value


use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::path::{Path, self};
use std::io::{self,prelude::*};


use crossbeam_channel::at;
use syn::{self, Stmt, Local, Pat, PatIdent, Expr, MethodTurbofish, TypeReference, GenericArgument, TypePath};
use super::parse_var::{node, VarInfo,stmt_node_type, block_node_type, FuncInfo};
use super::adjlist::Adjlist;

// arg 文件名
// return value methodmap


// 返回的 (i32,usize) i32 = 0 self ; i32 = 1 &self ；i32 = 2 mut self; i32 = 3 &mut self;
// return value: usize = 0 无/owner 1 :& 2:&mut
// 同时要获得所有的函数返回值



// 将methodlist提前存入
fn get_method_list(methodmap: &mut HashMap<String,(i32,usize)>){
    // len() &self->usize ()
    
    {let name_len = String::from("len");
    let method_self = 1;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // insert() &mut self -> () 
    {let name_len = String::from("insert");
    let method_self = 3;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}


    // push(&mut self
    {let name_len = String::from("push");
    let method_self = 3;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // get(&self -> Option&
    {let name_len = String::from("get");
    let method_self = 1;
    let return_value = 1;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // get_mut(&mut self -> Option&mut
    {let name_len = String::from("get_mut");
    let method_self = 3;
    let return_value = 2;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // clone(&self -> Self
    {let name_len = String::from("clone");
    let method_self = 1;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // as_str(&self -> &str
    {let name_len = String::from("as_str");
    let method_self = 1;
    let return_value = 1;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // is_empty(&self -> bool
    {let name_len = String::from("is_empty");
    let method_self = 1;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // to_vec(&self -> 
    {let name_len = String::from("to_vec");
    let method_self = 1;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}
    
    // last_mut(&mut self -> &mut
    {let name_len = String::from("last_mut");
    let method_self = 3;
    let return_value = 2;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // clear(&mut self 
    {let name_len = String::from("clear");
    let method_self = 3;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // as_bytes(& self ->& 
    {let name_len = String::from("as_bytes");
    let method_self = 1;
    let return_value = 1;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // nth(&mut self ->
    {let name_len = String::from("nth");
    let method_self = 3;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // iter(&self 
    {let name_len = String::from("iter");
    let method_self = 1;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // iter_mut(&mutself 
    {let name_len = String::from("iter_mut");
    let method_self = 3;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // to_owned(&self 
    {let name_len = String::from("to_owned");
    let method_self = 1;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // is_some(&self 
    {let name_len = String::from("to_owned");
    let method_self = 1;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // last(&self) -> Option<&T>
    {let name_len = String::from("last");
    let method_self = 1;
    let return_value = 1;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // unwrap(self)
    {let name_len = String::from("unwrap");
    let method_self = 0;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    // remove(&mut self)
    {let name_len = String::from("remove");
    let method_self = 3;
    let return_value = 0;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    {let name_len = String::from("as_ptr");
    let method_self = 3;
    let return_value = 10;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}

    {let name_len = String::from("as_mut_ptr");
    let method_self = 3;
    let return_value = 10;
    methodmap.insert(name_len.to_string(), (method_self, return_value));}


}






// 根据返回其usize  

pub fn get_type(type_refer: &syn::Type) -> usize{
    match  type_refer {
        syn::Type::Reference(TypeRefer) =>{
            if let Some(_) = TypeRefer.mutability{
                return 2;
            }
            return 1;
        }
        syn::Type::Path(TypePath) =>{

            if get_type_path(TypePath) != 0{
                return get_type_path(TypePath);
            }
            
        }
        syn::Type::Tuple(Typetuple) =>{
            
            for type_tuple in &Typetuple.elems{
                if get_type(type_tuple) != 0{

                    return get_type(type_tuple);
                }
            }
            
        }
        _ => ()
    }
    
    0
}



pub fn get_type_path(type_refer: &syn::TypePath) -> usize{
    
    for pathseg in &type_refer.path.segments{
        match &pathseg.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                for genericargument in &args.args {
                    match genericargument {
                        syn::GenericArgument::Type(newtype) => {
                            match  newtype {
                                syn::Type::Reference(TypeRefer) =>{
                                    if let Some(_) = TypeRefer.mutability{
                                        return 2;
                                    }
                                    return 1;
                                }
                                syn::Type::Path(TypePath) =>{

                                    if get_type_path(TypePath) != 0{
                                        return get_type_path(TypePath);
                                    }
                                    
                                }
                                syn::Type::Tuple(Typetuple) =>{
                                    
                                    for type_tuple in &Typetuple.elems{
                                        if get_type(type_tuple) != 0{
                                            return get_type(type_tuple);
                                        }
                                    }
                                    
                                }
                                _ => ()
                            }
                        }
                        _ =>()
                    }
                }
                
            }
            _ => ()
        }
    }
    0
}




pub fn method_call_names(path_name: &str) -> HashMap<String,(i32,usize)>{

    let mut methodmap = HashMap::new();
    
    let mut file = File::open(Path::new(path_name))
        .expect("method call Open file failed");
    
    let mut content = String::new();
    
    file.read_to_string(&mut content);

   

    let ast = syn::parse_file(&content)
        .expect("ast failed");
    
    // 获取 AST
    // 从 ast中读取所有 struct 
    // 要求struct 必须在最外层定义 作用域全局 不能定义在内部


    for item in &ast.items{
        // 构建h
        match item{
            syn::Item::Fn(ItemFn) => {
                // 搜索method
                let mut funcname = String::from(format!("{}",ItemFn.sig.ident)) ;
                // func 的i32 无意义
                let mut return_value = 0;
                // 获取return type
                match &ItemFn.sig.output {
                    syn::ReturnType::Default =>{
                        return_value =0;
                    }
                    syn::ReturnType::Type(_,Box_type ) =>{
                        match Box_type.as_ref() {
                            syn::Type::Reference(TypeRefer) =>{
                                if let Some(_) = TypeRefer.mutability{
                                    return_value =2;
                                }
                                return_value =1;
                            }
                            syn::Type::Path(TypePath) =>{
                                
                                return_value = get_type_path(TypePath);
                            }
                            syn::Type::Tuple(Typetuple) =>{
                                    
                                for type_tuple in &Typetuple.elems{
                                    if get_type(type_tuple) != 0{
                                        return_value =get_type(type_tuple);
                                    }
                                }
                            }
                            syn::Type::Ptr(typeptr) => {
                                return_value = 10;
                            }
                            _ =>()
                        }
                    }
                    _ => ()
                }
                // 获取到了 return type
                // 插入
                methodmap.insert(funcname.to_string(), (-1, return_value));
                
            }
            _ => ()
        }
    }




    for item in &ast.items{
        
        match item{
            syn::Item::Impl(Itemimpl) => {
                // 搜索method

                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            // 需要获取的内容： name/ self /return value ?
                            
                            let mut method_name = String::from(format!("{}",itemMethod.sig.ident)) ;
                            // signature number
                            // todo修改为返回值

                            let mut return_value:usize = 0; 
                            // 获取return value
                            match &itemMethod.sig.output {
                                syn::ReturnType::Default =>{
                                    return_value =0;
                                }
                                syn::ReturnType::Type(_,Box_type ) =>{
                                    match Box_type.as_ref() {
                                        syn::Type::Reference(TypeRefer) =>{
                                            if let Some(_) = TypeRefer.mutability{
                                                return_value =2;
                                            }
                                            return_value =1;
                                        }
                                        syn::Type::Path(TypePath) =>{
                                            
                                            return_value = get_type_path(TypePath);
                                        }
                                        syn::Type::Tuple(Typetuple) =>{
                                                
                                            for type_tuple in &Typetuple.elems{
                                                if get_type(type_tuple) != 0{
                                                    return_value =get_type(type_tuple);
                                                }
                                            }
                                        }
                                        syn::Type::Ptr(typeptr) => {
                                            return_value = 10;
                                        }
                                        _ =>()
                                    }
                                }
                                _ => ()
                            }
                            


                            // 相同有个 return type 一模一样


                            // name
                            let mut method_self = -1;
                            // self
                            for arg_self in &itemMethod.sig.inputs {
                                match arg_self {
                                    syn::FnArg::Receiver(receiver) => {
                                        let mut if_ref = false;
                                        let mut if_mut = false;
                                        
                                        if let Some(_) = receiver.reference {
                                            if_ref = true;
                                        }
                                        if let Some(_) = receiver.mutability {
                                            if_mut = true;
                                        }
                                        
                                        if !if_ref && !if_mut{ method_self=0 }; // self
                                        if if_ref && !if_mut{ method_self=1 }; // &self
                                        if !if_ref && if_mut{ method_self=2 }; // mut self
                                        if if_ref && if_mut{ method_self=3 }; // &mut self
                                        break;
                                    }
                                    _ => ()
                                }
                            }

                            // return value ? todo:: 再进行讨论
                            methodmap.insert(method_name.to_string(), (method_self, return_value));
                        }
                        _ => ()
                    }
                    
                }
            }
            _ => ()
        }
    }

    get_method_list(&mut methodmap);

    methodmap

}

// 测试产生的method call names
#[test]
fn test_create_method_hashmap2() {
    let path_name = "./src/parse/tests/1.rs";
    let mut methodmap = method_call_names(path_name);
    println!("{:?}",methodmap);
}


#[test]
fn test_create_method_hashmap() {
    // 第一个值为method name 第二个为self 第三个为signature 数量
    let mut methodmap = HashMap::new();
    
    let path_name = "./src/graphcsv/code/test.rs";

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");

    // 获取 AST
    // 从 ast中读取所有 struct 
    // 要求struct 必须在最外层定义 作用域全局 不能定义在内部

    for item in &ast.items{
        
        match item{
            syn::Item::Impl(Itemimpl) => {
                // 搜索method

                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            // 需要获取的内容： name/ self/ signature number /return value ?
                            
                            let mut method_name = String::from(format!("{}",itemMethod.sig.ident)) ;
                            // signature number
                            let mut signature_number = itemMethod.sig.inputs.len(); 
                            // name
                            let mut method_self = -1;
                            // self
                            for arg_self in &itemMethod.sig.inputs {
                                match arg_self {
                                    syn::FnArg::Receiver(receiver) => {
                                        let mut if_ref = false;
                                        let mut if_mut = false;
                                        
                                        if let Some(_) = receiver.reference {
                                            if_ref = true;
                                        }
                                        if let Some(_) = receiver.mutability {
                                            if_mut = true;
                                        }

                                        if !if_ref && !if_mut{ method_self=0 }; // self
                                        if if_ref && !if_mut{ method_self=1 }; // &self
                                        if !if_ref && if_mut{ method_self=2 }; // mut self
                                        if if_ref && if_mut{ method_self=3 }; // &mut self

                                        
                                    }
                                    _ => ()
                                }
                            }

                            // return value ? todo:: 再进行讨论
                            methodmap.insert(method_name.to_string(), (method_self, signature_number));

                        }
                        _ => ()
                    }
                    
                }



            }
            _ => ()
        }
    }
    println!("{:?}", &methodmap);

}