#[allow(warnings)]

// 对于别名表，查找参数是否拥有别名表中的名称


use std::io::Read;
use std::path::{Path, self};
use std::fs::File;

use syn::{self, Stmt, Local, Pat, PatIdent, Expr, Type, PathSegment, PatType};
use super::parse_var::{node, VarInfo,stmt_node_type, block_node_type, FuncInfo};
use super::adjlist::Adjlist;
use super::method_call::method_call_names;
use std::collections::HashMap;
use super::alias_analysis::create_alias_hashmap;


static mut flagger_all: i32 = 1;

//放一个并查集在这里 用来做一个点集合并
// 这里尝试使用来返回
#[derive(Debug)]
struct UnionFind {
    // 父节点
    parents: Vec<usize>, 
    // 秩
    ranks: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parents: (0..n).collect(),
            ranks: vec![0; n],
        }
    }

    // 找到祖先节点
    fn find(&mut self, x: usize) -> usize {
        let x_p = self.parents[x];
        if x_p != x {
            self.parents[x] = self.find(x_p); 
        }
        self.parents[x] 
    }

    // 合并两个节点所在的集合
    fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        } 

        if self.ranks[x_root] > self.ranks[y_root] {
            self.parents[y_root] = x_root;
        } else if self.ranks[x_root] < self.ranks[y_root] {
            self.parents[x_root] = y_root;
        } else {
            self.parents[y_root] = x_root;
            self.ranks[x_root] += 1; 
        }
    }
}



// 获取所有应该加入varmap的 varname
// 对两个string之间进行

// 传入一个别名表，传入一个当前分析的expr
// 返回即可
fn memoryzero_find(expr: &syn::Expr, alias_map: &HashMap<String,(usize,i32)>) -> i32 {
    match expr {
        Expr::Reference(exprRef) => {
            return memoryzero_find(&exprRef.expr.as_ref(),alias_map);
        }
        Expr::Path(exprpath) => {
            println!("12313123123132");
            let ask_map = String::from(format!("{}",  &exprpath.path.segments[0].ident));
            // 获取名字之后 
            if let Some(name_) = alias_map.get(&ask_map){
                
                return 2;
            }else {return 1;}
        }
        Expr::Call(exprcall) => {
            // 找到是否存在参数
            let mut flag = 1;
            for arg in &exprcall.args{
                if memoryzero_find(arg,alias_map)==2{
                    flag=2;
                }
            }
            return flag;
        }
        Expr::MethodCall(exprmethodcall)=> {
            //  只对引用进行一个集合合并是否正确

            let mut flag = 1;
            for arg in &exprmethodcall.args{
                if memoryzero_find(arg,alias_map)==2{
                    flag=2;
                }
            }
            return flag;

 
         }
         Expr::Cast(exprcast) => {
            return memoryzero_find(&exprcast.expr.as_ref(),alias_map);
         }
        _ => {
            return 1;
        }


    }

    return 1;
    
}



fn alias_union_this_expr(expr: &syn::Expr,  alias_of_this: &mut Vec<usize> ,union_find: &mut UnionFind ,alias_map: &HashMap<String,(usize,i32)> ,return_value_map: &HashMap<String, (i32, usize)>) 
{
    match expr{
        Expr::Reference(exprRef)=> {
            alias_union_this_expr(&exprRef.expr.as_ref(), alias_of_this, union_find, alias_map, return_value_map);
        }
        Expr::Path(exprpath) => {
            // 解path 只要第一个
            let lenth = exprpath.path.segments.len();
            // 获取名称之后找var_map
            let ask_map = String::from(format!("{}",  &exprpath.path.segments[0].ident));
            if let Some(name_) = alias_map.get(&ask_map){
                
                alias_of_this.push(name_.0.clone());
            }
            
        }
        Expr::MethodCall(exprmethodcall)=> {
           //  只对引用进行一个集合合并是否正确
            
            let flag = memoryzero_find(expr,alias_map);

            if flag == 2{
                unsafe{
                    flagger_all = 2;
                }
            }
            let method_name = String::from(format!("{}", exprmethodcall.method));
            if let Some(return_value) = return_value_map.get(&method_name){
                if return_value.1 >= 0{
                    // 是引用
                    // 解method reciever args
                    alias_union_this_expr(&exprmethodcall.receiver.as_ref(), alias_of_this, union_find, alias_map, return_value_map);

                    for arg in &exprmethodcall.args{


                        alias_union_this_expr(arg, alias_of_this, union_find, alias_map, return_value_map);

                    }

                }// 否则就不管了
            }

        }

        Expr::Call(exprcall) => {

            let flag = memoryzero_find(expr,alias_map);

            if flag == 2{
                unsafe{
                    flagger_all = 2;
                }
            }

            if let Expr::Path(exprpath) = &*exprcall.func{

                let func_name = String::from(format!("{}", exprpath.path.segments[0].ident));
                if let Some(return_value) = return_value_map.get(&func_name){
                    if return_value.1 >= 0{
                        // 是引用
                        // 解method reciever args

                        for arg in &exprcall.args{
                            alias_union_this_expr(arg, alias_of_this, union_find, alias_map, return_value_map);

                        }

                    }// 否则就不管了
                }
            }
        }
        Expr::Field(exprfeild)=>{
            match &exprfeild.base.as_ref(){
                Expr::Path(exprpath) =>{
                    for pathSegment in &exprpath.path.segments{
                        // 这里也是选择base里的第一个
                        let ask_map = String::from(format!("{}", pathSegment.ident));
                        if let Some(name_) = alias_map.get(&ask_map){
                            alias_of_this.push(name_.0.clone());
                        }
                    }
                }
                _=>()
            }
        }
        // let a = c+b;
        Expr::Binary(exprbinary) => {
            alias_union_this_expr(&exprbinary.left.as_ref(), alias_of_this, union_find, alias_map, return_value_map);
            alias_union_this_expr(&exprbinary.right.as_ref(), alias_of_this, union_find, alias_map, return_value_map);
        }
        Expr::Index(exprindex) => {
            // 别名分析 需要加上 
            alias_union_this_expr(&exprindex.expr.as_ref(), alias_of_this, union_find, alias_map, return_value_map);
        }
        Expr::Paren(exprparen) => {
            // 找结构体第一个 目的是找到base
            alias_union_this_expr(&exprparen.expr.as_ref(), alias_of_this, union_find, alias_map, return_value_map);

        }
        Expr::Try(exprtry) => {
            // 找结构体第一个 目的是找到base
            alias_union_this_expr(&exprtry.expr.as_ref(), alias_of_this, union_find, alias_map, return_value_map);

        }
        Expr::Struct(exprstruct) => {
            // 找结构体第一个 目的是找到base
            for struct_file in &exprstruct.fields{
                alias_union_this_expr(&struct_file.expr, alias_of_this, union_find, alias_map, return_value_map);
            }
            if let Some(rest_expr) = &exprstruct.rest{
                alias_union_this_expr(rest_expr.as_ref(), alias_of_this, union_find, alias_map, return_value_map);
            }
            
        }
        _=>()
    }
}


fn merge_union(union_find: &mut UnionFind,vec1: &Vec<usize>,vec2: &Vec<usize>){
    for i in vec1.iter(){
        for j in vec2.iter(){
            union_find.union(i.clone(), j.clone());
            // println!("hhhhhhhhhhhhhh");
        }
    }
}

// 解析expr
// alias_map 代表的是变量名字 以及在hashmap中的出现顺序，用以并查集使用
fn alias_union_expr(expr: &syn::Expr, union_find: &mut UnionFind ,alias_map: &HashMap<String,(usize,i32)> ,return_value_map: &HashMap<String, (i32, usize)>) {
    match expr{

        Expr::Call(exprcall) => {

            let flag = memoryzero_find(expr,alias_map);

            if flag == 2{
                unsafe{
                    flagger_all = 2;
                }
            }
        }
        Expr::MethodCall(exprcall) => {

            let flag = memoryzero_find(expr,alias_map);

            if flag == 2{
                unsafe{
                    flagger_all = 2;
                }
            }
        }

        Expr::Assign(exprassign) => {
            // assign 表达式
            let mut vec1 = Vec::<usize>::new();
            let mut vec2 = Vec::<usize>::new();

            alias_union_this_expr(&exprassign.left.as_ref(), &mut vec1, union_find,alias_map,return_value_map);
            alias_union_this_expr(&exprassign.right.as_ref(), &mut vec2, union_find,alias_map,return_value_map);
            // 获取两个之间的别名 
            // 互相添加
            merge_union(union_find,&vec1,&vec2);
        }
        Expr::AssignOp(exprassignop) => {
            // no need
        }
        Expr::Reference(exprRef)=> {
            
            alias_union_expr(&exprRef.expr.as_ref(),union_find, alias_map,return_value_map);
        }
        Expr::Block(expr_block) => {
            for stmt in &expr_block.block.stmts{
                alias_union_stmt(stmt, union_find, alias_map,return_value_map);
            }
        }
        
        Expr::Let(exprlet) => {
            // 关键语句
            let mut vec1 = Vec::<usize>::new();
            let mut vec2 = Vec::<usize>::new();

            alias_union_this_pat(&exprlet.pat, &mut vec1, union_find,alias_map,return_value_map);
            alias_union_this_expr(&exprlet.expr.as_ref(), &mut vec2, union_find,alias_map,return_value_map);
            // 获取两个之间的别名 
            // 互相添加
            merge_union(union_find,&vec1,&vec2);


        }
        Expr::ForLoop(exprforloop) => {
            let mut vec1 = Vec::<usize>::new();
            let mut vec2 = Vec::<usize>::new();
            alias_union_this_pat(&exprforloop.pat, &mut vec1, union_find,alias_map,return_value_map);
            alias_union_this_expr(&exprforloop.expr.as_ref(), &mut vec2, union_find,alias_map,return_value_map);
            // 获取两个之间的别名 
            // 互相添加
            merge_union(union_find,&vec1,&vec2);


            for stmt in &exprforloop.body.stmts{
                alias_union_stmt(stmt, union_find, alias_map,return_value_map);
            }

        }
        Expr::While(exprwhile) => {
            
            alias_union_expr(&exprwhile.cond.as_ref(),union_find, alias_map,return_value_map);

            for stmt in &exprwhile.body.stmts{
                alias_union_stmt(stmt, union_find, alias_map,return_value_map);
            }
        }
        Expr::Loop(exprloop) => {
            for stmt in &exprloop.body.stmts{
                alias_union_stmt(stmt, union_find, alias_map,return_value_map);
            }
        }
        Expr::If(Exprif) => {
            
            alias_union_expr(&Exprif.cond.as_ref(),union_find, alias_map,return_value_map);

            for stmt in &Exprif.then_branch.stmts{
                alias_union_stmt(stmt, union_find, alias_map,return_value_map);
            }// 有else 解 if
            if let Some((_,elseif)) = &Exprif.else_branch{
                alias_union_expr(&elseif.as_ref(),union_find, alias_map,return_value_map);
            }
        }
        Expr::Match(exprmatch) => {

            let mut vec2 = Vec::<usize>::new();
            alias_union_this_expr(&exprmatch.expr.as_ref(), &mut vec2, union_find,alias_map,return_value_map); 

            for arm in &exprmatch.arms{
                // 获取到pat 
                // alias_expr_release(&exprmatch.expr.as_ref(), &arm.pat, let_var, var_map, graph, return_value_map);
                // guard
                let mut vec1 = Vec::<usize>::new();
                alias_union_this_pat(&arm.pat, &mut vec1, union_find,alias_map,return_value_map);

                merge_union(union_find,&vec1,&vec2);


                if let Some((_,guard_expr)) = &arm.guard{
                    // 这里是个判断语句 ？
                    alias_union_expr(&guard_expr.as_ref(),union_find, alias_map,return_value_map);
                }
                //body
                alias_union_expr(&arm.body.as_ref(),union_find, alias_map,return_value_map);
            }

            // alias_expr_release(exprmatch.expr.as_ref(), pat, let_var, var_map, graph, return_value_map)

        }
        _=>()
    }
}

// 也是返回一个对于patident的解析
// alias_of_this 获取当前pat所有的 变量名称 并且可能由于临时变量的原因导致变成& 比如 method call 里面的 &元素，
// 如果左端是引用 那么需要获得右侧所有的引用 包括base
// 如果左端是owner 那么就需要右侧全部加上

// 
fn alias_union_this_pat(pat: &syn::Pat, alias_of_this: &mut Vec<usize> ,union_find: &mut UnionFind ,alias_map: &HashMap<String, (usize,i32)> ,return_value_map: &HashMap<String, (i32, usize)>)
{
    match pat {
        Pat::Ident(patident) => {
            // 加入 alias of this
            let name_ = patident.ident.to_string();
            if let Some(name_) = alias_map.get(&name_) {
                // 需要push进去哪些
                alias_of_this.push(name_.0.clone());
            }

        },
        Pat::Reference(pat_reference) => {
            // 到达reference节点需要继续递归
            alias_union_this_pat(&pat_reference.pat.as_ref(), alias_of_this, union_find,alias_map,return_value_map);
        },
        // 对于tuple只需要递归下去直到 ident/ref 就可以获取并创建了
        Pat::Tuple(pattuple) => {
            for new_pat in &pattuple.elems{
                alias_union_this_pat(&new_pat, alias_of_this, union_find,alias_map,return_value_map);
            }
            // tuple/tuplestruct需要递归
            // alias_name_tuple_expr(pattuple, &let_var, var_map, graph,return_value_map);
            // tuple_flag = true;
        }// 别的？
        Pat::Type(patype) => {
            // 可以管后面的type 可以不管
            alias_union_this_pat(&patype.pat.as_ref(), alias_of_this, union_find,alias_map,return_value_map);

        }// Some(a) 解开path 可以不管path
        Pat::TupleStruct(tupelstruct) =>{
            for new_pat in &tupelstruct.pat.elems{
                alias_union_this_pat(&new_pat, alias_of_this, union_find,alias_map,return_value_map);
            }
            
        }
        Pat::Slice(patslice) => {
            for new_pat in &patslice.elems{
                alias_union_this_pat(&new_pat, alias_of_this, union_find,alias_map,return_value_map);
            }
        }
        _ => {
            
        }
    }
}

// self 怎么加进去
fn alias_union_stmt(stmt: &syn::Stmt, union_find: &mut UnionFind ,alias_map: &HashMap<String, (usize,i32)> ,return_value_map: &HashMap<String, (i32, usize)>) {
    match stmt {
        Stmt::Local(loc) => {
            // 对于local语句 解析 左右连边 
            let mut vec1 = Vec::<usize>::new();
            let mut vec2 = Vec::<usize>::new();

            alias_union_this_pat(&loc.pat, &mut vec1, union_find,alias_map,return_value_map);
            if let Some((_,init_expr)) = &loc.init{
                alias_union_this_expr(init_expr.as_ref(), &mut vec2, union_find,alias_map,return_value_map);
            }
             // println!("vec1:{:?}  vec2:{:?}",vec1,vec2);
            // 获取两个之间的别名 
            // 互相添加
            merge_union(union_find,&vec1,&vec2);
        }
            // 初始化后的expr 中获取letvar信息
        Stmt::Semi(expr, _) =>{ 
            alias_union_expr(expr, union_find, alias_map, return_value_map);
        }
        Stmt::Expr(expr) => {
            alias_union_expr(expr, union_find, alias_map, return_value_map);
        }
        _ => ()
    }
}

fn alias_Steensgaard(ast: &syn::File,alias_map: &HashMap<String, (i32, bool, bool)> ,return_value_map: &HashMap<String, (i32, usize)>, funcname: &str)
    -> ( UnionFind ,HashMap<String, (usize, i32)>)
{
    let mut union_find = UnionFind::new(alias_map.capacity());

    // 扫ast
    let mut hashmap_key = HashMap::new();
    let mut i:usize = 0;
    for (alias_name, value_ref) in alias_map.iter(){
        // 获取出现顺序 以及对应的value是owner还是ref
        // v.0 是出现顺序
        hashmap_key.insert(alias_name.to_string(), (i, value_ref.0));
        i = i+1;
    }

    for item in &ast.items{
        match item{
            syn::Item::Fn(func) => {

                if func.sig.ident == funcname{
                    for stmt in &func.block.stmts {    
                        // 从let 开始加入所有语句 主要是搜索所有的let语句，找到所有的定义变量，再进行加边
                        // 先把所有的出现的名称加入varmap中
                        alias_union_stmt(stmt,&mut union_find ,&hashmap_key,return_value_map);
                        
                        
                    }
                }
            
            }
            syn::Item::Impl(Itemimpl) =>{
                for method in &Itemimpl.items{
                    match method {
                        syn::ImplItem::Method(itemMethod) => {
                            // method ident 
                            if itemMethod.sig.ident == funcname.to_string() {
                                // 进入

                                for stmt in &itemMethod.block.stmts {    
                                    // 从let 开始加入所有语句 主要是搜索所有的let语句，找到所有的定义变量，再进行加边
                                    // 先把所有的出现的名称加入varmap中
                                    alias_union_stmt(stmt,&mut union_find ,&hashmap_key,return_value_map);
                                    
                                }

                            }
                        }
                        _ =>()
                    }
                }
            }
            _ => ()
        }
    } 

    

    // 目标创建一个新的aliasmap，
    // 返回一个并查集表示alias依赖关系
    // 返回一个hashmap存储了alias的位置
    (union_find,hashmap_key)
}

//对外接口 ，
// 暂且没发现bug
// 能够根据 worng_variable filepath fucntion  返回一个新的hashmap 
pub fn get_union(worng_variable: &Vec<String>, path_name:&str, function_name: &str)->HashMap<String,(i32,bool,bool)>{
    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    
    let mut content = String::new();
    file.read_to_string(&mut content);

    
    let ast = syn::parse_file(&content).expect("ast failed");

    // 需要一个邻接表图表示？还是一个简单的并查集？
    // 最后返回一个hashmap?
    // 获取返回值
    let mut return_value_map = method_call_names(path_name);
    // 获取变量map
    let alias_map = create_alias_hashmap(path_name,function_name);

    

    // 根据 alias_map 大小创建一个并查集
    let (mut unionfind,index_alias_map) = alias_Steensgaard(&ast,&alias_map,&return_value_map,&function_name);

    // 返回 原 aliasmap的一部分
    let mut new_aliasmap = HashMap::<String,(i32,bool,bool)>::new();

    // 对于所有能在并查集中找到相同数字的alias 进行加入

    for variable in worng_variable {
        if let Some((index, _)) = index_alias_map.get(variable){
            let parent = unionfind.find(index.clone());
            for (alia_name , (alias_index,_)) in index_alias_map.iter() {
                // 如果同属一集 就加入 newaliasmap
                if unionfind.find(alias_index.clone()) == parent {
                    if let Some(value) = alias_map.get(alia_name){
                        new_aliasmap.insert(alia_name.to_string(), (value.0,value.1,value.2));
                    }
                }
            }
        }
    }



    new_aliasmap
}

#[test]
pub fn memoryzeros(){
    let path_name = "./src/parse/tests/1.rs";
    let name = "main";
    let mut variable_set = Vec::new();
    variable_set.push("myst".to_string());

    // 传入
    // 遍历一遍 找到所有fn函数参数中是否含有
    let d = get_steengaard_alias(path_name,name,&variable_set);
    println!("{:?}",d);

    unsafe {println!("flag:::{}",flagger_all);}

}



pub fn get_steengaard_alias(path_name:&str, function_name: &str, variable_set: &Vec<String>) 
        -> HashMap<String,(i32,bool,bool)>
{
    let mut file = File::open(Path::new(path_name))
        .expect("Steengaard Open file failed");
    let mut content = String::new();
    file.read_to_string(&mut content);

    
    let ast = syn::parse_file(&content).expect("ast failed");
    
    let mut return_value_map = method_call_names(path_name);
    // 获取变量map
    let alias_map = create_alias_hashmap(path_name,function_name);

    let (mut unionfind,new_alias_map) = alias_Steensgaard(&ast,&alias_map,&return_value_map,&function_name);

    let mut new_alias_map_after_union = HashMap::<String,(i32,bool,bool)>::new();

    for var_target in variable_set{
        if let Some((number_var,_)) = new_alias_map.get(var_target){
            let var_parrent = unionfind.find(number_var.clone());

            // 获取parent之后 找到所有的parent是这个string的加入新map里         
            for (name_alias,numbers) in new_alias_map.iter() {
                if unionfind.find(numbers.0.clone()) == var_parrent {
                    // print!("{},",name_alias);
                    // 把namealias parent等于 var_parrent的加进去
                    if let Some(target_get) = alias_map.get(name_alias){
                        
                        let value = (target_get.0,target_get.1,target_get.2);
                    
                        new_alias_map_after_union.insert(name_alias.to_string(), value);
                    } 
                    
                }
            }
                


        }
        




    }

    new_alias_map_after_union

}


#[test]
fn test_alias2(){

    let path_name = "./src/parse/tests/1.rs";
    let name = "some_f";
    let mut variable_set = Vec::new();
    variable_set.push("v".to_string());
    variable_set.push("self".to_string());
    
    let d = get_steengaard_alias(path_name,name,&variable_set);
    println!("{:?}",d);
}

#[test]
fn test_alias()
{
    let path_name = "./src/parse/tests/1.rs";
    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    // funcname
    let name = "some_f";
    
    let mut content = String::new();
    file.read_to_string(&mut content);

    
    let ast = syn::parse_file(&content).expect("ast failed");

    // 需要一个邻接表图表示？还是一个简单的并查集？
    // 最后返回一个hashmap?
    // 获取返回值
    let mut return_value_map = method_call_names(path_name);
    // 获取变量map
    let alias_map = create_alias_hashmap(path_name,name);


    // 根据 alias_map 大小创建一个并查集
    let (mut unionfind,new_alias_map) = alias_Steensgaard(&ast,&alias_map,&return_value_map,&name);
    // 创建一个新的别名表 类似于aliasmap 
    println!("aliasmap {:?}",new_alias_map);
    println!("aliasmap {:?}",new_alias_map.capacity());
    // let exist = HashMap::new();
    for i in 0..new_alias_map.capacity(){
        print!("{{");
        for (name_alias,numbers) in new_alias_map.iter() {
            if unionfind.find(numbers.0.clone()) ==i {
                print!("{},",name_alias);
            }
        }
        print!("}}");
    }

}

