


use std::collections::{ HashMap};
use std::fs::File;
use std::path::Path;
use std::io::{self,prelude::*};
use std::ptr::read;
use std::{vec, string};

use proc_macro2::Span;
use csv;
use proc_macro2::Ident;
use quote::ToTokens;
use quote::spanned::Spanned;


use syn::token::Continue;
use proc_macro2::TokenStream;
use crate::parse::steengaard;

use super::adjlist::Adjlist;
use super::alias_analysis;
use super::method_call::method_call_names;

use log::{debug, info};

use syn::{self, Stmt, Local, Pat, PatIdent, Expr, Signature, ExprReference, ExprBreak};

static mut flagger_all: i32 = 0;
static mut flagger_all_method: i32 = 0;

// 11.17 bug 为什么max的定义语句会出现两次啊。。。

// 枚举当前变量的类型，不在乎类型本身
// 是否存在问题？
// 变量 基本单位 

#[derive(Clone,Debug)]
pub struct VarInfo{
    // string 变量的名称
    pub Name: Option<String>,
    pub Reference: bool,
    pub Mutability: bool,
    pub Number: usize, // Number 表示在图中的编号
    // 关注变量的类型
    // todo::变量类型 结构体
}

// 函数语句，考虑名字以外还有变量？ 参数/返回值？
#[derive(Clone,Debug)]

// function call 与 methodcall合并
pub struct FuncInfo {
    pub Name: Option<String>,
    pub return_value: usize, //  返回值用什么表示？    0:没有返回值/返回值非引用 1:返回值& 2:&mut
    pub Number:usize,
    pub Start: bool,
    pub End: bool,
    pub method_call: i32,
    // method_call -1表示不是方法调用 0表示self 1表示&self 2表示 mut self 3表示 &mut self
}
// 新增返回值 传参数量变成返回值
// 对signature需要解path tuple typereference

// 只在函数内部的cfg 要考虑函数间调用，但是视作一个图中的普通节点
// 这是对单个函数做的分析，先不考虑其他函数

// node
#[derive(Clone,Debug)]
pub enum stmt_node_type {
    Owner(VarInfo),
    MutRef(VarInfo),
    StaticRef(VarInfo),
    PTR(VarInfo),
    Function(FuncInfo),
}

#[derive(Clone,Debug)]
pub enum block_node_type{
    BLOCK_START,
    BLOCK_END,
    BLOCK_NONE,
}



// 图中的一个节点应当是block类型或者是普通语句类型
// block 代表括号引起的scope分割
// stmt 代表一个变量的使用/声明等等
#[derive(Clone,Debug)]
// Todo ：return 节点
pub struct node{

    pub stmt: Option<stmt_node_type>,
    pub block: Option<block_node_type>,
}

// block节点插入

// graph.push(node { 
//     stmt: None, 
//     block: Some(block_node_type::BLOCK_START) 
// });
// graph.add(graph.len_num()-2, graph.len_num()-1);


impl node {
    pub fn new(stmt:Option<stmt_node_type>, block : Option<block_node_type>) -> node{
        node { stmt, block }
    }
}


fn reader() -> Vec<String> 
{
    // 当前假设所有出现错误的var 名字都叫va
    //let path = "tester.rs";
    let file = File::open(Path::new("./src/parse/tester.rs"))
        .expect("Open file failed");
    // 读入成功
    // lines 是一个迭代器 从第0行开始，next是第一行 
    let mut line_iter = io::BufReader::new(file).lines();
    // 第一行 
    let mut lines = line_iter.next()
        .expect("Read file failed")
        .expect("Read file failed");
    // println!("{:?}",lines);
    // vec lines 保存了整个文档的所有代码到一个vec里面，
    // 暂时没有搞多个文档
    let mut vec_lines: Vec<String> = Vec::new(); 
    vec_lines.push(lines);
    while true{
        
        
        lines = line_iter.next()
            .expect("reading wrong")
            .expect("character wrong");
        // println!("{:?}",lines);
        // 文档以 // end为结尾
        if lines == "// end"{
            break;
        }
        vec_lines.push(lines);
    }
    //
    // println!("{:?}", vec_lines);
    vec_lines
}

// 获取到了文档的每一行后进行处理

// 1 找到所有va的别名 包括(函数签名 + 函数返回值 + 变量内部方法) 
// 2 别名在同一个域的出现顺序 以及所在域 -> 识别 {} 和函数 ()
// 3 给别名做域的区分，包括函数传参和函数参数 函数返回值
//  先最简单版 不判断任何类型? (类型很重要，需要区分copy等等)
//  这个可以先适合 e502?

//
fn deal_vec_lines(vec_lines: Vec<String>){
    // 第一步 ， 先遍历找到va
    // 分离单行 找到变量
    
    let mut a = 1;
    // va 出现的行数
    let mut va_line_number = 0;

    // 提取词汇
    for s in vec_lines.iter(){
        let fields: Vec<&str> = s //  || c == ','?
            .split(|c| c == ' '  || c == '{' || c == '}')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        // println!("{:?}",fields);
        // ***遍历每一行的 fileds 先找到变量va 假设找到va的第一次出现
        // 每一行的fields 代表去掉空格之后的一个单词
        for s2 in fields{
            if s2 == "//" || s2 == "/*" {
                break;
            }
            if s2 == "va" {
                va_line_number = a;
                println!("found va");
                break;
            }
            
        }
        a = a + 1;
    
    }
    // 假设这里va是报错位置，已经获得了va的报错位置
    // 需要寻找va的定义
    // println!("{}",va_line_number);

}

#[test]
fn deal_test() {
    let v = reader();
    // 获取到了每一行的内容 a是当前的数字
    let a= deal_vec_lines(v);
}

// 处理一个hash表 获取是所有method name 以及返回值？  返回值意义?


// 测试读入文件 把文件转换成字符串存入
#[test]
fn reader_test() 
{
    // 当前假设所有出现错误的var 名字都叫va
    //let path = "tester.rs";
    let file = File::open(Path::new("./src/parse/tester.rs"))
        .expect("Open file failed");
    // 读入成功
    // lines 是一个迭代器 从第0行开始，next是第一行 
    let mut line_iter = io::BufReader::new(file).lines();
    // 第一行 
    let mut lines = line_iter.next()
        .expect("Read file failed")
        .expect("Read file failed");
    // println!("{:?}",lines);
    // vec lines 保存了整个文档的所有代码到一个vec里面，
    // 暂时没有搞多个文档
    let mut vec_lines: Vec<String> = Vec::new(); 
    vec_lines.push(lines);
    while true{

        lines = line_iter.next()
            .expect("reading wrong")
            .expect("character wrong");
        // println!("{:?}",lines);
        // 文档以 // end为结尾
        if lines == "// end"{
            break;
        }
        vec_lines.push(lines);
    }
    //
    // println!("{:?}", vec_lines);

}





// 解析syn 生成图 提取attribute? // 假设已经知道 所需变量？
fn graph_generate(ast: &syn::File, funcname: String, var_set: &mut HashMap<String,(i32,bool,bool)>/*  别名表*/, method_map: &HashMap<String,(i32,usize)>/*methodinfo */, askfunction: &str) -> Adjlist  {
    // 遍历item

    let mut graph = Adjlist::new();

    for item in &ast.items{
        // match fn 对于fn对象来构成图 // 寻找函数名称
        match item{
            syn::Item::Fn(func) => {
                // println!("{:?}",func.sig.ident);

                if func.sig.ident == askfunction.to_string(){

                    if let Some((_,return_value)) = method_map.get(&askfunction.to_string()){
                        graph.push_func_node(&askfunction.to_string(), true, false, return_value.clone());
                        // todo? 需要传入前后节点吗?
                        // 用节点标号构图 节点标号不能有错误

                        // signature

                        let mut continue_now = 0 as usize;
                        let mut break_vec = Vec::<i32>::new();
                        let mut return_vec = Vec::<usize>::new();

                        for arg in &func.sig.inputs {
                            match arg {
                                syn::FnArg::Typed(pattyped) => {
                                    // patident 是名字
                                    match &*pattyped.pat{
                                        Pat::Ident(patident) => {
                                            if var_set.contains_key(&String::from(format!("{}",patident.ident))){
                                                
                                                let var_str = String::from(format!("{}",patident.ident));
                                                if let Some(value) = var_set.get(&var_str) {
                                                    graph.push_node(value, &var_str);
                                                    graph.add(graph.len_num()-2, graph.len_num()-1);
                                                }
                                                else { println!("wrong value of hashmap");}
                                
                                            }
                                        }
                                        _ => {}
                                    }
                                    
                                    
                                }
                                _ => {}
                            }
                        } 

                        for stmt in &func.block.stmts {
                            // 传入图 别名表
                            let graph_num = graph.len_num();
                            
                            graph_stmt(&stmt , var_set, &mut graph, method_map , graph_num-1,&mut continue_now,&mut break_vec,&mut return_vec);

                        }
                        
                        graph.push_func_node(&askfunction.to_string(), false, true,return_value.clone());
                        graph.add(graph.len_num()-2, graph.len_num()-1);
                        // 结束时添加所有return节点
                        while !return_vec.is_empty(){
                            if let Some(u) = return_vec.pop(){
                                graph.add(u, graph.len_num()-1);
                            }
                        }
                        
                    }
                }
                
                

            }
            // 函数在impl块内
            syn::Item::Impl(Itemimpl) => {
                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            if itemMethod.sig.ident == askfunction.to_string() {
                                // 如果是方法的
                                if let Some((method_self_get,return_value_get)) = method_map.get(&askfunction.to_string()){
                                    let mut continue_now = 0 as usize;
                                    let mut break_vec = Vec::<i32>::new();
                                    let mut return_vec = Vec::<usize>::new();
                                    
                                    graph.push_method_node(&askfunction.to_string(), true, false, method_self_get.clone(), return_value_get.clone());
                                    // graph.push(node { stmt:Some(stmt_node_type::Function(FuncInfo{Name: Some(askfunction.to_string()), return_value: 0 , Number: graph.len_num(), Start:true, End:false, method_call: -1})) , block: None });
                                    for arg in &itemMethod.sig.inputs {
                                        match arg {
                                            syn::FnArg::Typed(pattyped) => {
                                                // patident 是名字
                                                match &*pattyped.pat{
                                                    Pat::Ident(patident) => {
                                                        if var_set.contains_key(&String::from(format!("{}",patident.ident))){
                                                            
                                                            let var_str = String::from(format!("{}",patident.ident));
                                                            if let Some(value) = var_set.get(&var_str) {
                                                                graph.push_node(value, &var_str);
                                                                graph.add(graph.len_num()-2, graph.len_num()-1);
                                                            }
                                                            else { println!("wrong value of hashmap");}
                                            
                                                        }
                                                    }
                                                    _ => {}
                                                } 
                                            }
                                            _ => {}
                                        }
                                    } 
                                

                                // signature

                                    for stmt in &itemMethod.block.stmts {
                                        let graph_num = graph.len_num();
                                        graph_stmt(&stmt , var_set, &mut graph, method_map , graph_num-1,&mut continue_now,&mut break_vec,&mut return_vec);
                                    }
                                    graph.push_method_node(&askfunction.to_string(), false, true, method_self_get.clone(), return_value_get.clone());
                                    graph.add(graph.len_num()-2, graph.len_num()-1);
                                    while !return_vec.is_empty(){
                                        if let Some(u) = return_vec.pop(){
                                            graph.add(u, graph.len_num()-1);
                                        }
                                    }
                                }
                            }
                        }
                        
                        _ => ()
                    }
                    
                }
            }
            _ => () // 
        }
    }   
    // test
    // graph.show();

    // println!("{:#?}",graph);
    graph

}

// 解析节点插入

fn graph_block (block: &syn::Block, var_def: &mut HashMap<String,(i32,bool,bool)>,graph: &mut Adjlist, method_map: &HashMap<String,(i32,usize)> ,
    last_node_num: usize,
    continue_now:&mut  usize,
    break_vec: &mut Vec<i32>,
    return_vec: &mut Vec<usize>,
){
    // 建图需要 前后加block节点

    // Todo： stmt中可能没有任何hashset变量出现，可能不需要插入任何节点 是否需要stmt中不进行任何节点插入操作，
    // 暂且不考虑不出现的问题，等待图构建完成后再进行搜索修图？
    graph.push_block_start();
    graph.add(last_node_num, graph.len_num()-1);
    //
    for stmt in &block.stmts {
        graph_stmt(stmt, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec); 
        // 存在可能性stmt中没有任何一个节点需要添加？
    }
    //
    graph.push_block_end();
    graph.add(graph.len_num()-2, graph.len_num()-1);
}
// 普通的pat
fn graph_pat (pat: &syn::Pat, var_def: &mut HashMap<String,(i32,bool,bool)>,graph: &mut Adjlist, last_node_num: usize)
{
    match pat {

        Pat::Ident(patident) => {
            
            if var_def.contains_key(&String::from(format!("{}",patident.ident))){
                
                let var_str = String::from(format!("{}",patident.ident));
                unsafe {
                    if flagger_all==1||flagger_all_method ==1{
                        if let Some(value) = var_def.get(&var_str) {
                            
                            if value.0==1 {
                                println!("Owner：{}",&String::from(format!("{}",&var_str)));
                            }else if value.0==2|| value.0==3{
                                println!("引用：{}",&String::from(format!("{}",&var_str)));
                            }else {
                                println!("指针：{}",&String::from(format!("{}",&var_str)));
                            }


                        }
                        if flagger_all==0 {
                            flagger_all=1;
                        }
                        if flagger_all_method==0 {
                            flagger_all_method=1;
                        }

                    }else {
                        flagger_all=1;
                        flagger_all_method =1;
                    }
                }
                



                // let var_str = String::from(format!("{}",patident.ident));
                if let Some(value) = var_def.get(&var_str) {
                    graph.push_node(value, &var_str);
                    graph.add(last_node_num, graph.len_num()-1);
                }
                else { println!("wrong value of hashmap");}

            }
        }
        Pat::Reference(pat_reference) => {
            // 到达reference节点需要继续递归
            graph_pat(&pat_reference.pat, var_def, graph, last_node_num);
        },
        Pat::Type(patype) => {
            // 可以管后面的type 可以不管
            graph_pat(&patype.pat, var_def, graph, last_node_num);

        } // 只有第一个是lastnodenum
        Pat::TupleStruct(tupelstruct) =>{

            for new_pat in &tupelstruct.pat.elems{

                graph_pat(&new_pat, var_def, graph, last_node_num);
            }
            
        }
        Pat::Tuple(pattuple) =>{
            for new_pat in &pattuple.elems{
                graph_pat(&new_pat, var_def, graph, last_node_num);
            }
            
        }
        Pat::Slice(patslice) => {
            for new_pat in &patslice.elems{
                graph_pat(&new_pat, var_def, graph, last_node_num);
            }
        }
        _=> {}
    }
}


// func名称解析
fn path_fmt(exprpath : &syn::ExprPath) -> String {
    let mut pathname = "".to_owned();
    for seg in exprpath.path.segments.iter() {
        pathname.push_str(&seg.ident.to_string());
        pathname.push_str(&String::from("::"));
        // println!("{:?}", seg);
    }
    pathname[0..pathname.len()-2].to_string()
}



// 解reference 有可能是其他表达式，需要解开
// 这里解的时候必须全部变成新的 ref mutability 
fn graph_expr_reference(
    expr: &syn::Expr,
    mutable:bool,
    var_def: &mut HashMap<String,(i32,bool,bool)>,
    graph: &mut Adjlist, 
    method_map: &HashMap<String,(i32,usize)>,
    last_node_num: usize,
    continue_now:&mut  usize,
    break_vec: &mut Vec<i32>,
    return_vec: &mut Vec<usize>,

) {
    // 获取一个reference，对其pat进行解析 将其对应的变量求解
    // 
    // 
    match expr{
        Expr::Reference(exprRef)=> {
            let mut new_mutable = false;
            if let Some(_) = &exprRef.mutability{
                new_mutable = true;
            }  
            graph_expr_reference(&exprRef.expr,new_mutable,var_def,graph,method_map,last_node_num,continue_now,break_vec,return_vec);
        }
        Expr::Path(exprpath) => {
            if let Some(var_name) = exprpath.path.get_ident(){
                //如果名称在hashset之内就可以进行存储
                if var_def.contains_key(&String::from(format!("{}",var_name))){
                    // 必须是ref

                    unsafe {
                        if flagger_all==1||flagger_all_method ==1{
                            println!("引用：{}",&String::from(format!("{}",var_name)));
                            if flagger_all==0 {
                                flagger_all=1;
                            }
                            if flagger_all_method==0 {
                                flagger_all_method=1;
                            }
                        }else {
                            flagger_all=1;
                            flagger_all_method =1;
                        }
                    }
                    

                    if let Some(_) = var_def.get(&String::from(format!("{}",var_name))) {
                        let mut new_value = (-1,false,false);
                        if mutable{
                            new_value = (2,true,true);
                        }else{
                            new_value = (3,true,false);
                        }
                        graph.push_node(&new_value, &String::from(format!("{}",var_name)));
                        graph.add(last_node_num, graph.len_num()-1);
                    }
                    else { println!("wrong value of hashmap");}

                }
            } 

            
        }
        Expr::Field(exprfeild)=>{
            match &exprfeild.base.as_ref(){
                Expr::Path(exprpath) =>{
                    if let Some(var_name) = exprpath.path.get_ident(){
                        //如果名称在hashset之内就可以进行存储
                        if var_def.contains_key(&String::from(format!("{}",var_name))){
                            // 获取元组 从三个变量中获得 需要push的节点的信息
                            if let Some(_) = var_def.get(&String::from(format!("{}",var_name))) {
                                let mut new_value = (-1,false,false);
                                if mutable{
                                    new_value = (2,true,true);
                                }else{
                                    new_value = (3,true,false);
                                }
                                graph.push_node(&new_value, &String::from(format!("{}",var_name)));
                                graph.add(last_node_num, graph.len_num()-1);
                            }
                            else { println!("wrong value of hashmap");}
        
                        }
                    }
                }
                _=>()
            }
        }
        // let a = c+b;
        Expr::Index(exprindex) => {
            // 别名分析 需要加上 
            graph_expr_reference(&exprindex.expr,mutable,var_def,graph,method_map,last_node_num,continue_now,break_vec,return_vec);
        }
        Expr::Paren(exprparen) => {
            // 找结构体第一个 目的是找到base
            graph_expr_reference(&exprparen.expr,mutable,var_def,graph,method_map,last_node_num,continue_now,break_vec,return_vec);

        }
        Expr::Try(exprtry) => {
            // 找结构体第一个 目的是找到base
            graph_expr_reference(&exprtry.expr,mutable,var_def,graph,method_map,last_node_num,continue_now,break_vec,return_vec);

        }
        // 其他的正常继续
        _=>{
            graph_expr(expr, var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);
        }
    }
}

// todo 需要建立一个 return节点 

// // 每一个if branch的开头都需要需要连接 lastnode
// fn graph_if_expr(expr_if: &syn::Expr,
//     var_def: &mut HashMap<String,(i32,bool,bool)>,
//     graph: &mut Adjlist, 
//     method_map: &HashMap<String,(i32,usize)>,
//     last_node_num: usize){
        
//     }



// 对一个表达式进行构图，关键是寻找出现的变量是否在hashset之中\
fn graph_expr(expr: &syn::Expr, var_def: &mut HashMap<String,(i32,bool,bool)>,
    graph: &mut Adjlist, 
    method_map: &HashMap<String,(i32,usize)>,
    last_node_num: usize,
    continue_now:&mut  usize,
    break_vec: &mut Vec<i32>,
    return_vec: &mut Vec<usize>,
){
    // 表达式可能出现多个变量的情况
    match expr {
        // 函数调用
        Expr::Call(exprcall)=> {
            // 放入函数调用的前提是，函数签名中带有hashmap存储的key值
            // signature
            // 对于建图 目前考虑和block类似 开头结尾构建func起点以及终点 对于owner需要考虑特殊标注？
            // 首先建立func节点
            // 对于建立func节点 
            

            // dongyan 打印call函数,前提参数内部有目标
            


            if let Expr::Path(exprpath) = &*exprcall.func{
                
                

                let func_nmae = &format!("{}", path_fmt(&exprpath));
                // println!("{}",func_nmae);
               // if let Some(_)  = method_map.get(func_nmae) {


                    let mut return_value = 0;
                    if let Some(fun_info) = method_map.get(&exprpath.path.segments[0].ident.to_string()){
                        return_value = fun_info.1.clone();
                    }
                    
                    // 这里把整个func接进去了 其实不用 比如一个没有备注过的function可以不管？
                    // 检查returnvalue
                    graph.push_func_node(&format!("{}", path_fmt(&exprpath)), true, false, return_value);
                    graph.add(last_node_num, graph.len_num()-1);

                    

                    // 1 代表owner； 2代表mut ref ；3代表immutref；接下来是 ref 和 mutability
                    for arg in &exprcall.args { // dongyan版本，打印出现的每一个arg
                        // println!("12312312");
                        graph_expr(arg, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
                    }

                    graph.push_func_node(&format!("{}", path_fmt(&exprpath)), false, true,return_value);
                    graph.add(graph.len_num()-2, graph.len_num()-1);

                    // 节点构建后遍历其signature
               // }
            }
            
            unsafe {
                if flagger_all == 1 {
                    
                    // println!("123123123123123132312");      
                    if let Expr::Path(exprpath) = &*exprcall.func{
                        println!("");
                        println!("-------------------------");
                        print!("函数");
                        for path_ident in &exprpath.path.segments {
                            print!("{}::",path_ident.ident.to_string());
                        }
                        println!("");

                        // 1 代表owner； 2代表mut ref ；3代表immutref；接下来是 ref 和 mutability
                        for arg in &exprcall.args { // dongyan版本，打印出现的每一个arg
                            graph_expr(arg, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
                        }
                        
                        println!("函数结束");
                        println!("-------------------------");
                        println!("");

                        flagger_all = 0;
                        
                    }
                }
            }


            
            

        }
        // 方法调用
        Expr::MethodCall(exprmethod) => {

            // dongyan 打印call函数,前提参数内部有目标

            // methodcall 节点与func call相同 创建methodcall节点 并且遍历其arg
            // method 节点插入
            let method_name = &String::from(format!("{}",exprmethod.method));
            let lastone = graph.len_num()-1;
            graph_expr(&exprmethod.receiver, var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);

            let nowone = graph.len_num()-1;
            
            if let Some(method_info) = method_map.get(method_name){
                // method_info.0是self选项 method_info.1是arg number
                // 找到当前method
                // 
                
                graph.push_method_node(method_name, true, false, method_info.0 ,method_info.1);
                
                if lastone == nowone{
                    graph.add(last_node_num, graph.len_num()-1);
                }else {
                    graph.add(graph.len_num()-2, graph.len_num()-1);
                }

                

                // 获取了当前的method 还需要插入一个self节点？ 隐式调用了self 把self放进去 插入一个self名字的节点
                let mut statement = (0,false,false);
                if method_info.0 == 0 {
                    statement.0 = 1;
                    statement.1 = false;
                    statement.2 = false;
                }else if method_info.0 == 1 {
                    statement.0 = 3;
                    statement.1 = true;
                    statement.2 = false;
                }else if method_info.0 == 2 {
                    statement.0 = 1;
                    statement.1 = false;
                    statement.2 = true;
                }else if method_info.0 == 3 {
                    statement.0 = 2;
                    statement.1 = true;
                    statement.2 = true;
                }
                let var_str = "self".to_string();
                graph.push_node(&statement, &var_str);
                graph.add(graph.len_num()-2, graph.len_num()-1);



                for arg in &exprmethod.args {
                    graph_expr(arg, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
                }
                graph.push_method_node(method_name, false, true, method_info.0, method_info.1);
                graph.add(graph.len_num()-2, graph.len_num()-1);
            }



            unsafe {
                if flagger_all_method == 1 {
                    
                    // println!("123123123123123132312");      
                    if let Expr::Path(exprpath) = &*exprmethod.receiver{
                        println!("");
                        println!("-------------------------");

                        print!("方法");
                        println!("{}",exprmethod.method.to_string());
                        print!("方法对象是一个");
                        for path_ident in &exprpath.path.segments {

                            let var_str = path_ident.ident.to_string();

                            if var_def.contains_key(&var_str){
                

                                
                                    
                            if let Some(value) = var_def.get(&var_str) {
                                
                                if value.0==1 {
                                    print!("Owner：{}",&String::from(format!("{}",&var_str)));
                                }else if value.0==2|| value.0==3{
                                    print!("引用：{}",&String::from(format!("{}",&var_str)));
                                }else {
                                    print!("指针：{}",&String::from(format!("{}",&var_str)));
                                }
    
    
                            }
                
                                    
                                
                            }
                        }
                        
                        println!("");
                        // 1 代表owner； 2代表mut ref ；3代表immutref；接下来是 ref 和 mutability
                        for arg in &exprmethod.args { // dongyan版本，打印出现的每一个arg
                            graph_expr(arg, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
                        }
                        
                        println!("方法结束");
                        println!("-------------------------");
                        println!("");

                        flagger_all_method = 0;
                        
                    }
                }
            }



            // 优先处理一个method_call hash表 根据表进行处理

        }
        Expr::Index(exprindex) => {
            let lastone = graph.len_num()-1;
            graph_expr(&exprindex.index, var_def, graph, method_map, last_node_num,continue_now,break_vec,return_vec);
            let nowone = graph.len_num()-1;
            if lastone!=nowone{
                graph_expr(&exprindex.expr, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);

            }
            else {
                graph_expr(&exprindex.expr, var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);
            }
        }
        Expr::Assign(exprassign) => {
            let lastone = graph.len_num()-1;
            graph_expr(&exprassign.right.as_ref(), var_def, graph, method_map, last_node_num,continue_now,break_vec,return_vec);
            let nowone = graph.len_num()-1;
            if lastone!=nowone{
                graph_expr(&exprassign.left.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);

            }
            else {
                graph_expr(&exprassign.left.as_ref(), var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);
            }
        }
        Expr::AssignOp(exprassignop) => {
            let lastone = graph.len_num()-1;
            graph_expr(&exprassignop.right.as_ref(), var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);
            let nowone = graph.len_num()-1;
            if lastone!=nowone{
                graph_expr(&exprassignop.left.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
            }
            else {
                graph_expr(&exprassignop.left.as_ref(), var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);

            }
        }
        Expr::Binary(exprbinary) => {
            let lastone = graph.len_num()-1;
            graph_expr(&exprbinary.right.as_ref(), var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);
            let nowone = graph.len_num()-1;
            if lastone!=nowone{
                graph_expr(&exprbinary.left.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
            }
            else {
                graph_expr(&exprbinary.left.as_ref(), var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);

            }        
        }
        Expr::Cast(exprcast) => {
            match &*exprcast.ty {
                syn::Type::Ptr(typeptr) => {
                    print!("指针");
                }
                _ => ()
            }
            graph_expr(&exprcast.expr.as_ref(), var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);
            
        }
        Expr::Block(exprblock) => {
            
            graph_block(&exprblock.block, var_def, graph, method_map, last_node_num,continue_now,break_vec,return_vec);

        }
        Expr::Struct(exprstruct) => {
            // struct 表达式与 pat相关
            for filed in &exprstruct.fields{
                graph_expr(&filed.expr, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);

            }
        }
        Expr::Reference(exprreference ) => {
            // 临时变量的产生 在这里会有变量变化的疑问，直接在在这里解开比较合适 解开method call

            // 临时变量 处理三个？path 
            // 有reference证明是存在 &需要跟上&
            // 这个必定是个临时变量 需要修改
            
            // 必须要根据reference后方的东西传递一个进去

            let mut mutable = false;
            if let Some(_) = &exprreference.mutability{
                mutable = true;
            }  
            // pat一个新的 
            graph_expr_reference(&exprreference.expr,mutable,var_def,graph,method_map,last_node_num,continue_now,break_vec,return_vec);

        }
        Expr::Field(exprfeild) => {
            graph_expr(&exprfeild.base,var_def,graph,method_map,last_node_num,continue_now,break_vec,return_vec);
        }
        Expr::If(exprif) => {
            // if 和else相连接

            // if 前的判断语句

            

            graph_expr (&exprif.cond.as_ref(), var_def, graph, method_map,last_node_num,continue_now,break_vec,return_vec);
            // 建立分支节点
            
            // block start 前的节点
            let if_start = graph.len_num()-1; // 这个节点需要连接else边

            graph_block(&exprif.then_branch, var_def, graph, method_map, if_start,continue_now,break_vec,return_vec);
            // graph.add(graph.len_num()-2, graph.len_num()-1);
            // 第一个branch的结尾 block节点
            let first_brach_end = graph.len_num()-1;
            
            // else branch后的 expr只可能是if 或者block 
            
            if let Some(else_branch_expr) = &exprif.else_branch{
                // 第二个block的前节点是 ifstart节点

                graph_expr(else_branch_expr.1.as_ref(), var_def, graph, method_map,if_start,continue_now,break_vec,return_vec);
                // 第二个branch尾节点
                let second_branch_end = graph.len_num()-1;
                // 创建一个空节点做if 的结尾

                graph.push_block_none();
                // 连接两个block尾节点和最后的空节点
                graph.add(second_branch_end, graph.len_num()-1);
                
                graph.add(first_brach_end, graph.len_num()-1);
            }else { // 如果没有elsebranch 那么需要连接一个从if expr 到结尾的节点
                graph.add(if_start, graph.len_num()-1);
            }   
            
            


            
        }
        // 如何处理循环的break 以及 continue？以及return？
        // 保存loop 起点应对continue
        // 在loop 终点连接所有break
        Expr::While(exprwhile) => {
            let v = graph.len_num();
            let tmp_continue = *continue_now;
            // 碰到循环，
            // 清理 break
            *continue_now = graph.len_num();
            break_vec.push(-1);

            graph_expr (&exprwhile.cond.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
            
            graph_block(&exprwhile.body, var_def, graph, method_map, graph.len_num()-1,continue_now,break_vec,return_vec);
            // blockend 连边
            let u = graph.len_num()-1;
            graph.add(u, v);
            // 所有break连边
            while !break_vec.is_empty() {
                if let Some(break_u) = break_vec.pop(){
                    if break_u==-1 {
                        break;
                    }
                    graph.add(break_u as usize, u);
                }

            }
            *continue_now = tmp_continue;
            
            // 在 loop while 期间传入参数变化，保留上一个

        }
        Expr::ForLoop(exprfor) => {
            // 循环在前后block加入一条返回边
            // pat需要单独解析！
            // Todo: 理解 pat 与siginature关系 
            let v = graph.len_num();
            let tmp_continue = *continue_now;
            // 碰到循环，
            // 清理 break
            *continue_now = graph.len_num();
            break_vec.push(-1);


            graph_pat(&exprfor.pat, var_def, graph, graph.len_num()-1);
            
            graph_expr (&exprfor.expr.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
            
            graph_block(&exprfor.body, var_def, graph, method_map, graph.len_num()-1,continue_now,break_vec,return_vec);

            let u = graph.len_num()-1;
            graph.add(u, v);
            // 所有break连边
            while !break_vec.is_empty() {
                if let Some(break_u) = break_vec.pop(){
                    if break_u==-1 {
                        break;
                    }
                    graph.add(break_u as usize, u);
                }

            }
            *continue_now = tmp_continue;
        }
        Expr::Loop(exprloop) => {
            let v = graph.len_num();
            let tmp_continue = *continue_now;
            // 碰到循环，
            // 清理 break coninue 是新的点
            *continue_now = graph.len_num();
            break_vec.push(-1);

            graph_block(&exprloop.body, var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
            let u = graph.len_num()-1;
            graph.add(u, v);
            // 所有break连边
            while !break_vec.is_empty() {
                if let Some(break_u) = break_vec.pop(){
                    if break_u==-1 {
                        break;
                    }
                    graph.add(break_u as usize, u);
                }

            }
            *continue_now = tmp_continue;
        }
        Expr::Let(exprlet) => {
            graph_pat(&exprlet.pat, var_def, graph, graph.len_num()-1);
            graph_expr (&exprlet.expr.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
        }
        Expr::Return(exprreturn) => {
            // 设立return节点
            // 所有的return都存储在一个returnvec之内，returnvec在函数结束时 吧所有的点都连接到mainblock上
            // 后面有个expr
            if let Some(expr_return_this ) =  &exprreturn.expr{
                graph_expr (expr_return_this.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);

            }
            return_vec.push(graph.len_num()-1);
        }

        Expr::Continue(exprcontinue) => {
            // 设立 continue 节点，从上一个传递过来的for循环节点，在loop退出时改回去 
            graph.add(graph.len_num()-1,continue_now.clone());
        }
        Expr::Break(ExprBreak) => {
            // 设立break节点 ，存储一个break数组，在loop结束时 吧所有的break连接到loop终点block上

            break_vec.push((graph.len_num()-1) as i32);
        }

        // 终结点 变量
        Expr::Path(exprpath) =>{
            // let var = exprpath.path.get_ident();
            
            if let Some(var_name) = exprpath.path.get_ident(){
                //如果名称在hashset之内就可以进行存储


                




                if var_def.contains_key(&String::from(format!("{}",var_name))){
                    // 获取元组 从三个变量中获得 需要push的节点的信息

                    let var_str = String::from(format!("{}",var_name));
                    unsafe {
                        if flagger_all==1||flagger_all_method ==1{
                            if let Some(value) = var_def.get(&var_str) {
                                
                                if value.0==1 {
                                    println!("Owner：{}",&String::from(format!("{}",&var_str)));
                                }else if value.0==2|| value.0==3{
                                    println!("引用：{}",&String::from(format!("{}",&var_str)));
                                }else {
                                    println!("指针：{}",&String::from(format!("{}",&var_str)));
                                }


                            }
                            if flagger_all==0 {
                                flagger_all=1;
                            }
                            if flagger_all_method==0 {
                                flagger_all_method=1;
                            }

                        }else {
                            flagger_all=1;
                            flagger_all_method =1;
                        }
                    }




                    if let Some(value) = var_def.get(&String::from(format!("{}",var_name))) {
                        graph.push_node(value, &String::from(format!("{}",var_name)));
                        graph.add(last_node_num, graph.len_num()-1);
                    }
                    else { println!("wrong value of hashmap");}

                }
            }   
        }
        // 其他暂不考虑

        Expr::Closure(exprclosure) => {
            // Todo 
        }
        Expr::Unsafe(exprunsafe) => {
            // Todo
            graph_block(&exprunsafe.block, var_def, graph, method_map, last_node_num,continue_now,break_vec,return_vec);
        }
        Expr::Match(exprmatch) => {
            // 
            graph_expr (&exprmatch.expr.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);

            graph.push_block_start();
           graph.add(graph.len_num()-2, graph.len_num()-1);
            let matchnode = graph.len_num()-1;

            let mut arm_end_vec = Vec::new();

            //
            // match 连边的时候必须是last node
            // 如果一个arm是空就不连接
            for arm in &exprmatch.arms{
                let arm_start = graph.len_num()-1;
                graph_pat(&arm.pat, var_def, graph, matchnode);
                // 连接节点 后续继续
                graph_expr(&arm.body.as_ref(), var_def, graph, method_map,graph.len_num()-1,continue_now,break_vec,return_vec);

                if arm_start!=graph.len_num()-1{
                    arm_end_vec.push((graph.len_num()-1).clone());
                }
                

            }

            // end结束 所有边都连block
            // 后续连边需要 在结尾需要连 所有arm结束的边以及 blockend
            graph.push_block_end();
            for arm_end in &arm_end_vec {
                graph.add(arm_end.clone(), graph.len_num()-1);
            }
            
            // graph.add(graph.len_num()-2, graph.len_num()-1)

            // 对arm如何
        }
        Expr::Macro(exprmacro) => {
            // 只看println 不管别的 mac path无用
            // println里必须是&
            let mut tokentree_buff = Vec::new();
            let mut first_lit = false;
            for item in exprmacro.mac.tokens.clone() {
                match item {
                    proc_macro2::TokenTree::Punct(punct) => {
                        if !first_lit {
                            tokentree_buff.clear();
                            first_lit =true;
                        }else {
                            let mut tokensteram = proc_macro2::TokenStream::new();
                            tokensteram.extend(tokentree_buff);
                            let res: Result<syn::Expr, syn::Error> = syn::parse2(tokensteram);
                            match  res{
                                Ok(expr_print) => {
                                    graph_expr_reference(&expr_print, true,var_def, graph, method_map, graph.len_num()-1,continue_now,break_vec,return_vec);
                                    
                                }
                                Err(_) => println!("erro macro"),
                            }
                            tokentree_buff = Vec::new();
                        }
                    }
                    _ => {
                        tokentree_buff.push(item);
                    }
                }
            }
            // for 循环结束再做一遍
            let mut tokenstream_buff = proc_macro2::TokenStream::new();
            tokenstream_buff.extend(tokentree_buff);
            let res: Result<syn::Expr, syn::Error> = syn::parse2(tokenstream_buff);
            match res {
                Ok(exp) => {
                    graph_expr_reference(&exp,false, var_def, graph, method_map, last_node_num,continue_now,break_vec,return_vec);
                    
                }
                Err(_) => debug!("Assert macro parse error"),
            }

        }
        _ => {}
    }

} // 


fn graph_stmt(stmt: &syn::Stmt, var_def: &mut HashMap<String,(i32,bool,bool)>, graph: &mut Adjlist,method_map: 
    &HashMap<String,(i32,usize)> , 
    last_node_num: usize,
    continue_now:&mut  usize,
    break_vec: &mut Vec<i32>,
    return_vec: &mut Vec<usize>,
){
    //stmt;别名表;图; 表示节点应该插入在哪个节点的后面 ？ 节点后的位置如何判定 连边？
    // 有多个

    // Todo ：varinfo中mut ref的信息没有更正 应该与hashset保存一直
    let mut varloc = VarInfo{Name: None, Mutability:false, Reference:false, Number: 0};

    match stmt{
        // 解析 let 表达式
        Stmt::Local(loc) =>{
            // let 语句 先判断名称后确定是否加入
            let mut pushornot = graph.len_num();
            
            // 赋值后面语句的表达式
            // 不能直接expr 与表达式有关

            // 获取


            if let Some((_eq, expr_stmt)) = &loc.init {
                graph_expr(expr_stmt, var_def, graph, method_map, last_node_num, continue_now, break_vec, return_vec);   
            }
            if pushornot == graph.len_num(){
                graph_pat( &loc.pat, var_def, graph, last_node_num);
            }else {
                graph_pat( &loc.pat, var_def, graph, graph.len_num()-1);
            }
            
            // 应当在hashset 中存储 mutablility 以及 reference内容
            // 节点编号可能会出问题？什么时候push?
            // 是否应当在结束后push/add
            // 还是let声明 不需要存储正在定义的变量 只需要后面出现的变量？
            // 这里需要后面出现的变量名称返回值改名为当前let语句中的identifyname 
        },
        Stmt::Semi(expr,_semi) => {
            graph_expr(expr, var_def, graph,method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
        },
        Stmt::Expr(expr) => {
            graph_expr(expr, var_def, graph,method_map,graph.len_num()-1,continue_now,break_vec,return_vec);
        }
        Stmt::Item(item) => {
            // Item fn?
        }



    }





}// 解析普通语句，进行图构建



#[test]
fn synparse_run() {
    // 跑一个case 查看跑出来的东西
    let path_name = "./src/parse/tests/1.rs";

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);
    // println!("{:?}",content);
    // 没有parsefile？
    let ast = syn::parse_file(&content)
        .expect("ast failed");
    // println!("{:#?}",ast); 打印ast
    // 目前假设函数名字就是main
    // 当前上下文不敏感
    
    let name ="next";

    let mut variable_set = Vec::new();
    variable_set.push("v".to_string());
    // variable_set.push("v".to_string());

    //variable_set.push("self".to_string());
    let mut var_set = steengaard::get_steengaard_alias(path_name, name, &variable_set);

    

    let method_map = method_call_names(path_name);
    // 打印别名表
    println!("别名表如下");
    println!("{:?}",var_set);
    println!("");
    // 打印方法表
    // println!("{:?}",method_map);


    // var_set.insert("my_array".to_string(),(1 as i32,false,false));
    // var_set.insert("max".to_string(),(1 as i32,false,false));
    // var_set.insert("min".to_string(),(1 as i32,false,false));
    //let name = String::from("main");
    // 生成并且打印图 获取图的样貌
    let graph = graph_generate(&ast, String::from("main"),&mut var_set, &method_map, &name);
    // 生成csv x / edge 向量
    
    println!("");
    // graph.show();


    
    
}




fn synparse_run_dongyan() {
    // 不需要进行判断，只需要扫一遍fn函数内的参数是否包含var里面的东西就行了

    let path_name = "./src/parse/tests/1.rs";

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);
    // println!("{:?}",content);
    // 没有parsefile？
    let ast = syn::parse_file(&content)
        .expect("ast failed");
    // println!("{:#?}",ast); 打印ast
    // 目前假设函数名字就是main
    // 当前上下文不敏感
    
    let name ="main";

    let mut variable_set = Vec::new();
    variable_set.push("my_struct".to_string());
    // variable_set.push("v".to_string());
    
    // 获取表之后，找到是否被fn调用



    let mut var_set = steengaard::get_steengaard_alias(path_name, name, &variable_set);

    

    let method_map = method_call_names(path_name);
    // 打印别名表
    println!("别名表如下");
    println!("{:?}",var_set);
    println!("");
    // 打印方法表
    // println!("{:?}",method_map);


    // var_set.insert("my_array".to_string(),(1 as i32,false,false));
    // var_set.insert("max".to_string(),(1 as i32,false,false));
    // var_set.insert("min".to_string(),(1 as i32,false,false));
    //let name = String::from("main");
    // 生成并且打印图 获取图的样貌
    let graph = graph_generate(&ast, String::from("main"),&mut var_set, &method_map, &name);
    // 生成csv x / edge 向量
    
    println!("");
    graph.show();

}




#[test]
fn test_csv_creat2(){

    let code_path = "./src/parse/tests/1.rs";
    let funcname ="f";
    let csv_path = "./src/parse/tests/1";
    let mut vars_vec = Vec::new();
    vars_vec.push("v".to_string());


    csv_creat2(code_path, csv_path, funcname, &vars_vec);

}


pub fn csv_creat2(code_path: &str, csv_path: &str, funcname: &str, vars_vec: &Vec<String>)
{
    // 目录是当前cmd下目录相对路径
    // 读取文件
    let mut file = File::open(Path::new(code_path))
        .expect("csv_creat open file failed ");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");
    // 图生成
    let method_map = method_call_names(code_path);
    
    let mut var_set = steengaard::get_steengaard_alias(code_path, funcname, vars_vec);


 //    let mut var_set = alias_analysis::create_alias_hashmap(code_path,funcname);
    // let name = String::from("main");
    
    
    let graph = graph_generate(&ast, funcname.to_string(),&mut var_set, &method_map, funcname);
    // 已经得到graph之后
    
    
    
    use csv::Writer;
    let xcsv = csv_path.to_string() + "x.csv";
    let edgecsv = csv_path.to_string() + "edge.csv";

    let mut wtr1 = Writer::from_path(&xcsv).expect("read csv wrong");
    let mut wtr2 = Writer::from_path(&edgecsv).expect("read csv wrong");
    // graph.show(); 函数体内部有bug
    // 生成x
    
    for i in 0..graph.len_num() {
        // 变量名称如何表示？
        let y = graph.vector_x_attribute(i);
        // 暂且不管function call的名字 只考虑变量名相同关系
        // 用hashmap中的顺位表示变量的string有待商榷
        let mut varnumber =0;
        // 如果是-1 就不操作
        if y.0 != "-1".to_string(){
            for key in var_set.keys() {
                if key.to_string() == y.0.to_string() {
                    break;
                }
                varnumber+= 1;
            }

        }else {
            varnumber = -1;
        }
        
        // 用methodmap中的顺位表示函数的String有待商榷
        let mut mehotdnumber =0;
        // 如果是node节点就不管
        if y.4 != "-1".to_string(){
            for key in method_map.keys() {
                if key.to_string() == y.4.to_string() {
                    break;
                }
                mehotdnumber+= 1;
            }
        }else {
            mehotdnumber = -1;
        }
        let x = (varnumber,y.1,y.2,y.3,mehotdnumber,y.5,y.6,y.7,y.8);        
        wtr1.write_record(&[x.0.to_string(), x.1.to_string(), x.2.to_string(),x.3.to_string(),x.4.to_string(),x.5.to_string(),x.6.to_string(),x.7.to_string(),x.8.to_string()])
            .expect("write_wrong");
    }

    for i in 0..graph.len_num() {
        let edge = graph.vector_edge_attribute(i);
        for e in edge {
            // println!("123123123");
            wtr2.write_record(&[i.to_string(), e.to_string()])
                .expect("write_wrong");
        }
    } 
    
    wtr1.flush().expect("write_wrong");
    wtr2.flush().expect("write_wrong");
    // 创建了两个csv表，一个是x向量，一个是edge向量


    // 输出到csv中
}




// 对于单个的code代码，通过graph_generate生成csv x/edge
pub fn csv_creat(code_path: &str, csv_path: &str, funcname: &str)
{
    // 目录是当前cmd下目录相对路径
    // 读取文件
    let mut file = File::open(Path::new(code_path))
        .expect("csv_creat open file failed ");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");
    // 图生成
    let method_map = method_call_names(code_path);

    let mut var_set = alias_analysis::create_alias_hashmap(code_path,funcname);
    // let name = String::from("main");
    let graph = graph_generate(&ast, funcname.to_string(),&mut var_set, &method_map, funcname);
    // 已经得到graph之后
    use csv::Writer;
    let xcsv = csv_path.to_string() + "x.csv";
    let edgecsv = csv_path.to_string() + "edge.csv";

    let mut wtr1 = Writer::from_path(&xcsv).expect("read csv wrong");
    let mut wtr2 = Writer::from_path(&edgecsv).expect("read csv wrong");
    // graph.show(); 函数体内部有bug
    // 生成x
    // todo ： 这里的节点有一定问题
    for i in 0..graph.len_num() {
        // 变量名称如何表示？
        let y = graph.vector_x_attribute(i);
        // 暂且不管function call的名字 只考虑变量名相同关系
        // 用hashmap中的顺位表示变量的string有待商榷
        let mut varnumber =0;
        for key in var_set.keys() {
            if key.to_string() == y.0.to_string() {
                break;
            }
            varnumber+= 1;
        }
        let x = (varnumber,y.1,y.2,y.3,-1,y.5,y.6,y.7);        
        wtr1.write_record(&[x.0.to_string(), x.1.to_string(), x.2.to_string(),x.3.to_string(),x.4.to_string(),x.5.to_string(),x.6.to_string(),x.7.to_string()])
            .expect("write_wrong");
    }
    for i in 0..graph.len_num() {
        let edge = graph.vector_edge_attribute(i);
        for e in edge {
            // println!("123123123");
            wtr2.write_record(&[i.to_string(), e.to_string()])
                .expect("write_wrong");
        }
    } 
    
    wtr1.flush().expect("write_wrong");
    wtr2.flush().expect("write_wrong");
    // 创建了两个csv表，一个是x向量，一个是edge向量


    // 输出到csv中
}
#[test]
fn csv_create_test2(){
    let a ="./spider_stackoverflow/src/dataok/code0/0.rs";
    let b = "./spider_stackoverflow/src/dataok/code0/0";
    let c = "f";
    csv_creat(a,b,c);
}

// 输出csv文件
#[test]
fn csv_create_test() 
{
    // 表示图的构建
    // todo: 批处理？生成多个x edge？
    use csv::Writer;
    // 先不考虑多个文件？

    // 读取源代码

    let path_name = "./src/graphcsv/code/test.rs";

    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    let mut content = String::new();
    file.read_to_string(&mut content);
    let ast = syn::parse_file(&content)
        .expect("ast failed");

    // 从ast中获取methodcall
    let method_map = method_call_names(path_name);
    let name ="main";
    let mut var_set = alias_analysis::create_alias_hashmap(path_name,name);
    let name = String::from("main");
    let graph = graph_generate(&ast, String::from("main"),&mut var_set, &method_map, &name);
    // 获取得到cfg
    // 暂时只考虑main函数
    // graph.show();

    let mut wtr1 = Writer::from_path("./src/graphcsv/csv/testx.csv").expect("read_wrong");
    let mut wtr2 = Writer::from_path("./src/graphcsv/csv/testedge.csv").expect("read_wrong");
    // 对graph进行解析 生成 x向量和 edge向量
    // 对每个节点生成x 与edge
    // x输出
    for i in 0..graph.len_num() {
        // 变量名称如何表示？
        let y = graph.vector_x_attribute(i);
        // 暂且不管function call的名字 只考虑变量名相同关系
        // 用hashmap中的顺位表示变量的string有待商榷
        let mut varnumber =0;
        for key in var_set.keys() {
            if key.to_string() == y.0.to_string() {
                break;
            }
            varnumber+= 1;
        }
        let x = (varnumber,y.1,y.2,y.3,-1,y.5,y.6,y.7);        
        wtr1.write_record(&[x.0.to_string(), x.1.to_string(), x.2.to_string(),x.3.to_string(),x.4.to_string(),x.5.to_string(),x.6.to_string(),x.7.to_string()])
            .expect("write_wrong");
    }
    for i in 0..graph.len_num() {
        let edge = graph.vector_edge_attribute(i);
        for e in edge {
            // println!("123123123");
            wtr2.write_record(&[i.to_string(), e.to_string()])
                .expect("write_wrong");
        }
    } 
    
    // wtr.write_record(&["a", "b", "c"]).expect("write_wrong");
    // wtr.write_record(&["x", "2", "z"]).expect("write_wrong");
    wtr1.flush().expect("write_wrong");
    wtr2.flush().expect("write_wrong");
    // 创建了两个csv表，一个是x向量，一个是edge向量

}



// 通过 funcname 获取 行数