

use std::io::Read;
use std::path::{Path, self};
use std::fs::File;
use syn::{self, Stmt, Local, Pat, PatIdent, Expr, Type, PathSegment, PatType};
use super::parse_var::{node, VarInfo,stmt_node_type, block_node_type, FuncInfo};
use super::adjlist::Adjlist;

use super::method_call::method_call_names;
use std::collections::HashMap;

// 从graph中找到对应节点的名字以及东西 返回一个owner ref mut的值
fn find_var(number_graph: usize, graph: &Adjlist) -> (i32,bool,bool){
    let mut return_property = (0 as i32, false,false);
    if let Some(stmtnode) = &graph.heads_list[number_graph].data.stmt {
        match  stmtnode {
            // 返回值
            stmt_node_type::Owner(varinfo) => {
                if let Some(a) = &varinfo.Name {
                    return_property.0 = 1;
                    return_property.1 = false;
                    return_property.2 = false;
                }
                
            }
            stmt_node_type::MutRef(varinfo) => {
                if let Some(a) = &varinfo.Name {
                    return_property.0 = 2;
                    return_property.1 = varinfo.Reference;
                    return_property.2 = varinfo.Mutability;
                }
            }
            stmt_node_type::StaticRef(varinfo) => {
                if let Some(a) = &varinfo.Name {
                    return_property.0 = 3;
                    return_property.1 = varinfo.Reference;
                    return_property.2 = varinfo.Mutability;
                }
            }
            _ => ()
            
        }
    }
    return_property
}



fn alias_opt_expr(expr: &syn::Expr, let_var: &mut (i32, bool, bool, String), var_map: &mut HashMap<String, usize>, graph: &mut Adjlist,return_value_map: &HashMap<String,(i32,usize)>) ->(i32, bool, bool, String){
    // 可以通过hashmap先找到
    // 可以传会一个let var 通过graph寻找已经存在的
    let mut opt = (1,false,false,"for".to_string());
    match expr{
        Expr::Reference(exprRef)=> {
            opt.1 = true;
            if let Some(_) = &exprRef.mutability {
                opt.2 = true;
                opt.0 = 2;
            }else{
                opt.0 = 3;
            }
        }
        Expr::Path(exprpath) => {
            // 解path 获取变量名称以及
            // 这里所见的opt只有第一个
            let lenth = exprpath.path.segments.len();
            // 获取名称之后找var_map
            let ask_map = String::from(format!("{}",  &exprpath.path.segments[0].ident));
            if let Some(number_graph) = var_map.get(&ask_map){
                let number_ = number_graph.clone();
                // property owner/ref  ref  mut
                let property = find_var(number_, graph);
                //
                opt.0 = property.0;
                opt.1 = property.1;
                opt.2 = property.2;
                opt.3 =  String::from(format!("{}", &exprpath.path.segments[0].ident));
            }
            
        }
        Expr::MethodCall(exprmethodcall)=> {
            // 使用returnvaluemap获取返回值
            // 获取method名称
            let method_name = String::from(format!("{}", exprmethodcall.method));
            if let Some(return_value) = return_value_map.get(&method_name){
                match return_value.1 {
                    0 => {
                        opt.0 = 1;
                    }
                    1 =>{
                        opt.0 = 3;
                        opt.1 = true;
                        opt.2 = false;
                    }
                    2 =>{
                        opt.0 = 2;
                        opt.1 = true;
                        opt.2 = true;
                    }
                    10 => {
                        opt.0 = 10;
                        opt.1 = true;
                        opt.2 = true;
                    }
                    _ => ()
                } 
            }
        }
        Expr::Call(exprcall) => {
            // 使用 return_value_map 
            // call 里面是个path path解到最后才是名字
            let mut func_name = "for".to_string();
            match &exprcall.func.as_ref() {
                Expr::Path(exprpath) => {
                    let lenth = exprpath.path.segments.len();
                    func_name = String::from(format!("{}", exprpath.path.segments[lenth-1].ident)); 
                }
                _ => ()
            }
        
            
            if let Some(return_value) = return_value_map.get(&func_name){
                match return_value.1 {
                    0 => {
                        opt.0 = 1;
                    }
                    1 =>{
                        opt.0 = 3;
                        opt.1 = true;
                        opt.2 = false;
                    }
                    2 =>{
                        opt.0 = 2;
                        opt.1 = true;
                        opt.2 = true;
                    }
                    10 => {
                        opt.0 = 10;
                        opt.1 = true;
                        opt.2 = true;
                    }
                    _ => ()
                } 
            }
        }
        Expr::Field(exprfeild)=>{
            match &exprfeild.base.as_ref(){
                Expr::Path(exprpath) =>{
                    for pathSegment in &exprpath.path.segments{
                        // 如果存在hashmap中就取出来
                        let ask_map = String::from(format!("{}", pathSegment.ident));
                        if let Some(number_graph) = var_map.get(&ask_map){
                            // 如果存在，就从graph其中找到 对应节点获取值
                            // 
                            let number_ = number_graph.clone();
                            // property owner/ref  ref  mut
                            let property = find_var(number_, graph);
                            //

                            opt.0 = property.0;
                            opt.1 = property.1;
                            opt.2 = property.2;
                            opt.3 =  String::from(format!("{}", pathSegment.ident));
                        }
                    }
                }
                _=>()
            }
        }
        // let a = c+b;
        Expr::Binary(exprbinary) => {
            // 这种情况只需要分析一个就可以了
            let property = alias_opt_expr( &exprbinary.left, let_var, var_map, graph, return_value_map);
            opt.0 = property.0;
            opt.1 = property.1;
            opt.2 = property.2;
            opt.3 =  String::from(format!("{}",property.3));
           
        }
        Expr::Index(exprindex) => {
            // 别名分析 需要加上 
            let property = alias_opt_expr( &exprindex.expr.as_ref(), let_var, var_map, graph, return_value_map);
            opt.0 = property.0;
            opt.1 = property.1;
            opt.2 = property.2;
            opt.3 =  String::from(format!("{}",property.3));
            
        }
        Expr::Cast(exprcast) => {
            // as?
        }
        Expr::Field(exprfeild) => {
            // 找结构体第一个 目的是找到base
            let property = alias_opt_expr( &exprfeild.base, let_var, var_map, graph, return_value_map);
            opt.0 = property.0;
            opt.1 = property.1;
            opt.2 = property.2;
            opt.3 =  String::from(format!("{}",property.3));
        }
        Expr::Paren(exprparen) => {
            // 找结构体第一个 目的是找到base
            let property = alias_opt_expr( &exprparen.expr.as_ref(), let_var, var_map, graph, return_value_map);
            opt.0 = property.0;
            opt.1 = property.1;
            opt.2 = property.2;
            opt.3 =  String::from(format!("{}",property.3));
        }
        Expr::Try(exprtry) => {
            // 找结构体第一个 目的是找到base
            let property = alias_opt_expr( &exprtry.expr.as_ref(), let_var, var_map, graph, return_value_map);
            opt.0 = property.0;
            opt.1 = property.1;
            opt.2 = property.2;
            opt.3 =  String::from(format!("{}",property.3));
        }
        _=>()
    }
    // 返回获得的值
    opt

}
// 分析出所有的pat带ref mut 直接找到对应的
// 递归tuple
// now_var 表示解析过程中的ref mut
fn alias_pat(exprpat: &syn::Pat, vec_alias: &mut Vec<(i32,bool,bool,String)>, mut var_now: (bool,bool)){ // 第一个ref 第二个mut
    // 从pat获取所有新创建的别名

    match exprpat{
        // 到达ident节点创建一个新的变量
        Pat::Ident(patident) => {
            let mut this_alias = (1,var_now.0,var_now.1,"for".to_string());
            
            this_alias.3 = String::from(format!("{}", patident.ident));
            if this_alias.3 == "None".to_string() {
                // 不管None
                return ;
            }
            this_alias.1 = true;
            if let Some(_mutable) = &patident.mutability {
                this_alias.2 = true;
                if let Some(_mutable) = &patident.by_ref {
                    // mut ref
                    this_alias.1 = true;
                    this_alias.0 = 2;
                }else {
                    // mut owner
                    this_alias.0 = 1;
                }
                
                
            }else{
                if let Some(_mutable) = &patident.by_ref {
                    this_alias.1 = true;
                    this_alias.0 = 3;
                }else {
                    this_alias.0 = 1;
                }
                
            }
            vec_alias.push(this_alias);
            
            // 创建一个变量塞入
        },
        Pat::Reference(pat_reference) => {
            // 到达reference节点需要继续递归
            var_now.0 = true;
            if let Some(_) = pat_reference.mutability{
                var_now.1= true;
            }
            alias_pat(&pat_reference.pat.as_ref(), vec_alias, var_now);
            
        },
        // 对于tuple只需要递归下去直到 ident/ref 就可以获取并创建了
        Pat::Tuple(pattuple) => {
            for new_pat in &pattuple.elems{
                alias_pat(&new_pat, vec_alias, var_now);
            }
            // tuple/tuplestruct需要递归
            // alias_name_tuple_expr(pattuple, &let_var, var_map, graph,return_value_map);
            // tuple_flag = true;
        }// 别的？
        Pat::Type(patype) => {
            // 可以管后面的type 可以不管

            match &*patype.ty {
                syn::Type::Ptr(typeptr) => {
                    let mut this_alias = (10,var_now.0,var_now.1,"for".to_string());
                    // 如果是ty 就是10
                    

                    match &*patype.pat{
                        syn::Pat::Ident(patident) => {
                            this_alias.3 = String::from(format!("{}", patident.ident));
                            vec_alias.push(this_alias);
                        }
                        _ => ()
                    }


                }
                _ =>{

                }
            }

            // alias_pat(&patype.pat.as_ref(), vec_alias, var_now);


        }// Some(a) 解开path 可以不管path
        Pat::TupleStruct(tupelstruct) =>{
            for new_pat in &tupelstruct.pat.elems{
                alias_pat(&new_pat, vec_alias, var_now);
            }
            
        }
        _ => {
            
        }
    }
}
// 将vecalias中所有的var放入varmap以及graph中
fn var_get_in(vec_alias: &Vec<(i32,bool,bool,String)>,var_map: &mut HashMap<String, usize>, graph: &mut Adjlist,return_opt: &(i32, bool, bool, String))
{   
    // 
    for var in vec_alias {
        let mut var_new = (return_opt.0,return_opt.1,return_opt.2);
        // 1 false false 是默认，如果不是默认就还回去，否则就是看右侧的返回值
        if var.0 !=1{var_new.0 =var.0}
        if var.1 !=false{var_new.1 =var.1}
        if var.2 !=false{var_new.2 =var.2}

        var_map.insert(String::from(format!("{}", var.3)), graph.len_num());
        graph.push_node(&(var_new.0,var_new.1,var_new.2), &var.3);
    }
    
}
// 获取pat的所有变量以及对应的变量内容
fn alias_expr_release(expr: &syn::Expr,pat: &syn::Pat,let_var: &mut (i32, bool, bool, String), var_map: &mut HashMap<String, usize>, graph: &mut Adjlist,return_value_map: &HashMap<String,(i32,usize)>){
    let return_opt = alias_opt_expr(expr, let_var, var_map, graph, return_value_map);
    // alias_name_expr(&exprlet.expr, &mut let_var2, var_map, graph,return_value_map);
    let mut vec_alias = Vec::<(i32,bool,bool,String)>::new();
    // 新建 插入
    // 从一个pat后面更新所有的东西
    let var_now = (false,false);
    alias_pat(pat, &mut vec_alias, var_now);
    // println!("{:?}",vec_alias);
    // 从 vec_alias 全部插入let中
    var_get_in(&vec_alias,var_map,graph,&return_opt);
}


fn alias_name_expr(expr: &syn::Expr, let_var: &mut (i32, bool, bool, String), var_map: &mut HashMap<String, usize>, graph: &mut Adjlist,return_value_map: &HashMap<String,(i32,usize)>){
    // 解析expr 获取letvar内容
    match expr{
        Expr::Reference(exprRef)=> {
            let_var.1 = true;
            if let Some(_) = &exprRef.mutability {
                let_var.2 = true;
            }
            alias_name_expr(exprRef.expr.as_ref(),let_var,var_map,graph,return_value_map);
        }
        Expr::Block(expr_block) => {
            for stmt in &expr_block.block.stmts{
                alias_name_stmt(stmt, graph, var_map,return_value_map);
            }

        }
        // let some(x) = ? 与for循环中存在变量的定义
        // let for 中具有相同问题 定义的变量需要判断后方的expr，
        
        Expr::Let(exprlet) => {
            alias_expr_release(&exprlet.expr, &exprlet.pat, let_var, var_map, graph, return_value_map);

        }
        Expr::ForLoop(exprforloop) => {
            // for loop 
            // 这里有个pat pat根据后面的expr获取内容
            
            alias_expr_release(&exprforloop.expr.as_ref(), &exprforloop.pat, let_var, var_map, graph, return_value_map);

            

            for stmt in &exprforloop.body.stmts{
                alias_name_stmt(stmt, graph, var_map,return_value_map);
            }

        }
        Expr::Unsafe(exprunsafe) =>{
            for stmt in &exprunsafe.block.stmts{
                alias_name_stmt(stmt, graph, var_map,return_value_map);
            }
        }
        Expr::While(exprwhile) => {
            alias_name_expr(exprwhile.cond.as_ref(),let_var,var_map,graph,return_value_map);
            for stmt in &exprwhile.body.stmts{
                alias_name_stmt(stmt, graph, var_map,return_value_map);
            }
        }
        Expr::Loop(exprloop) => {
            for stmt in &exprloop.body.stmts{
                alias_name_stmt(stmt, graph, var_map,return_value_map);
            }
        }
        Expr::If(Exprif) => {
            
            alias_name_expr(&Exprif.cond.as_ref(),let_var,var_map,graph,return_value_map);
            for stmt in &Exprif.then_branch.stmts{
                alias_name_stmt(stmt, graph, var_map,return_value_map);
            }// 有else 解 if
            if let Some((_,elseif)) = &Exprif.else_branch{
                alias_name_expr(&elseif.as_ref(),let_var,var_map,graph,return_value_map);
            }
        }
        Expr::Match(exprmatch) => {
            // match n { Some(n) => {}, None => {} } 
            // pub expr: Box<Expr>,
            // pub brace_token: Brace,
            // pub arms: Vec<Arm>,
            for arm in &exprmatch.arms{
                // 获取到pat 
                alias_expr_release(&exprmatch.expr.as_ref(), &arm.pat, let_var, var_map, graph, return_value_map);
                // guard
                if let Some((_,guard_expr)) = &arm.guard{
                    alias_name_expr(&guard_expr.as_ref(),let_var,var_map,graph,return_value_map);
                }
                //body
                alias_name_expr(&arm.body.as_ref(),let_var,var_map,graph,return_value_map);
            }

            // alias_expr_release(exprmatch.expr.as_ref(), pat, let_var, var_map, graph, return_value_map)

        }
        _=>()
    }
}

// pat + expr 因为tuple导致的递归循环问题，所以只能在 ident/ref 结尾而tuple需要继续递归 
// 此时因为expr已经解析过，已经获得了后续信息，只需解析pat内部即可
// fn alias_name_tuple_expr(tuple:&syn::PatTuple,let_var: &(i32, bool, bool, String), var_map: &mut HashMap<String, usize>, graph: &mut Adjlist,return_value_map: &HashMap<String,(i32,usize)>)
// {
//     // todo: tuple内容需要做特殊处理吗？
//     for element in &tuple.elems {
//         // pat 可能是tuple 可能不是
//         let mut let_var2 = (let_var.0, let_var.1, let_var.2, String::from(format!("{}", let_var.3)));
//         let mut tuple_flag = false;
//         match  &element{
//             Pat::Ident(patident) => {
//                 let_var2.3 = String::from(format!("{}", patident.ident));
//                 if let Some(_mutable) = &patident.mutability {
//                     let_var2.2 = true;
//                 }
//             },
//             Pat::Reference(pat_reference) => {
//                 if let Pat::Ident(pat_ident) = &*pat_reference.pat {

//                     let_var2.3 = String::from(format!("{}", pat_ident.ident));
//                     if let Some(_mutable) = &pat_reference.mutability {
//                         let_var2.2 = true;
//                     }
//                 }
//             },
//             // Todo tuble
//             Pat::Tuple(pattuple) => {
//                 tuple_flag = true;
//                 alias_name_tuple_expr(pattuple, let_var, var_map, graph,return_value_map);
//             }
//             _ => {}
//         }
//         if !tuple_flag{
//             if let_var2.1 {
//                 if let_var2.2 { // mut
//                     let_var2.0 = 2;
//                 } else {
//                     let_var2.0 =3 ;
//                 }
//             }else {
//                 let_var2.0 = 1; 
//             }
//             var_map.insert(String::from(format!("{}", let_var2.3)), graph.len_num());
//             graph.push_node(&(let_var2.0,let_var2.1,let_var2.2), &let_var2.3);
//         }

//     }

// }



// todo : 是否需要一个保存出现过别名的hashmap用来记录所有的变量以及标号？
fn alias_name_stmt(stmt: &syn::Stmt, graph: &mut Adjlist, var_map: &mut HashMap<String,usize>,return_value_map: &HashMap<String,(i32,usize)>) {
    // 搜寻所有的let语句 以及会出现定义名称的迭代器等，找到所有名称 获得其属性，插入graph
    // 实质是搜索所有let语句，找到其变量内容意义
    
    // 通过let后的表达式判断当前的 let语句创建的变量是什么样的变量
    // 第0个变量表示var当前属于 owner/staref/mutref 第1个变量表示是否是ref 第2个变量表示是否是mut 第3个表示变量名称
    
    let mut let_var = (0, false, false, "for".to_string());
    let mut tuple_flag = false;
    match stmt {
        Stmt::Local(loc) => {

            // 初始化后的expr 中获取letvar信息

            
            if let Some((_, expr))= &loc.init {
                alias_expr_release(expr.as_ref(), &loc.pat, &mut let_var, var_map, graph, return_value_map);
            }
            else {// 初始命名时 没有初始值，只能看type 并且带入&loc.pat

                let mut vec_alias = Vec::<(i32,bool,bool,String)>::new();
                let var_now = (false,false);
                alias_pat(&loc.pat, &mut vec_alias, var_now);

                let mut var_type = (1,false,false);
                // 从type获取新的知识

                match &loc.pat {
                    Pat::Type(pattype) => {
                        match pattype.ty.as_ref() {
                            syn::Type::Reference(typeRef) => {
                                var_type.1 = true;
                                if let Some(_) = typeRef.mutability {
                                    var_type.2 = true;
                                    var_type.0 = 2;
                                }else {
                                    var_type.0 =3;
                                }
                            }
                            _ => ()
                        }
                    }
                    _ => ()
                }
                // 获取type内容之后，将它转入 
                for var in vec_alias {
                    if var.0 !=1{var_type.0 =var.0}
                    if var.1 !=false{var_type.1 =var.1}
                    if var.2 !=false{var_type.2 =var.2}
                    // let mut var_new = (return_opt.0,return_opt.1,return_opt.2);
                    // // 1 false false 是默认，如果不是默认就还回去，否则就是看右侧的返回值
                    // if var.0 !=1{var_new.0 =var.0}
                    // if var.1 !=false{var_new.0 =var.0}
                    // if var.2 !=false{var_new.0 =var.0}
                    // 这里可能有问题
                    var_map.insert(String::from(format!("{}", var.3)), graph.len_num());
                    graph.push_node(&(var_type.0,var_type.1,var_type.2), &var.3);
                }
                // 从 vec_alias 全部插入let中
                
            }


        }
        Stmt::Semi(expr, _) =>{
            alias_name_expr(expr, &mut let_var, var_map, graph,return_value_map);
        }
        Stmt::Expr(expr) => {
            alias_name_expr(expr, &mut let_var, var_map, graph,return_value_map);
        }
        _ => ()
    }

}   



fn alias_map_genarate(ast: &syn::File, funcname: &str,return_value_map: &HashMap<String,(i32,usize)>) -> Adjlist {
    // graph 存储别名关系 连边
    let mut graph = Adjlist::new();
    // varmap负责查找variable对应名称的graph节点序号
    let mut var_map = HashMap::new();
    // 放入一个空节点
    var_map.insert("for".to_string(), 0); // 0节点是空的，什么也没有
    graph.push_node(&(0,false,false), &"for".to_string());

    
    // 与parsevar相同搜索ast 
    // todo 是否需要扫两遍？第一遍扫所有的
    for item in &ast.items{ 
        // 传入graph
        match item{
            syn::Item::Fn(func) => {
                // println!("{:?}",func.sig.ident);

                // Todo：for arg in &func.sig.inputs {} 暂时先不看main所有的signiture
                // todo 先只分析main函数
                // todo 暂且不考虑结构体
                if func.sig.ident == funcname{ //是否会出现情况导致无法进入分支条件？
                    // println!("123");
                    // 考虑signature
                    

                    for arg in &func.sig.inputs {
                        let mut let_var = (0, false, false, "for".to_string());
                        match arg {
                            syn::FnArg::Typed(pattyped) => {
                                // IDENTIFIER
                                match &*pattyped.pat{
                                    Pat::Ident(patident) => {
                                        let_var.3 = (String::from(format!("{}", patident.ident)));
                                    }
                                    _ => {}
                                } 
                                match &*pattyped.ty{
                                    Type::Reference(TyRef) =>{
                                        let_var.1 = true;
                                        
                                        if let Some(_mutability) = &TyRef.mutability{//mutref
                                            let_var.0 = 2;
                                            let_var.2 =true;
                                        }else {//staticref
                                            let_var.2 = false;
                                            let_var.0 = 3;
                                        }
                                    }
                                    Type::Path(typa) => {
                                        let_var.0 = 1;
                                        let_var.1 = false;
                                        let_var.2 = false;

                                    }
                                    // 如果是指针，letvar改成10后续表达指针
                                    Type::Ptr(typtr) => {
                                        let_var.0 = 10;
                                        let_var.1 = false;
                                        let_var.2 = false;

                                    }
                                    _ =>{}
                                }
                            }
                            // syn::FnArg::Typed(pattyped) => {

                            // }
                            _ => {}
                        }
                        graph.push_node(&(let_var.0,let_var.1,let_var.2), &let_var.3);
                    } 
                    // push节点
                    
                    for stmt in &func.block.stmts {    
                        // 从let 开始加入所有语句 主要是搜索所有的let语句，找到所有的定义变量，再进行加边
                        // 先把所有的出现的名称加入varmap中
                        alias_name_stmt(stmt, &mut graph, &mut var_map,return_value_map);
                        
                    }
                }

            }
            syn::Item::Impl(Itemimpl) =>{
                for method in &Itemimpl.items{
                    match &method {
                        syn::ImplItem::Method(itemMethod) => {
                            if itemMethod.sig.ident == funcname.to_string() {

                                graph.push(node { stmt:Some(stmt_node_type::Function(FuncInfo{Name: Some(funcname.to_string()), return_value: 0 , Number: graph.len_num(), Start:true, End:false, method_call: -1})) , block: None });
                                for arg in &itemMethod.sig.inputs {
                                    let mut let_var = (0, false, false, "for".to_string());
                                    match arg {
                                        
                                        syn::FnArg::Receiver(receiver) => {
                                            let_var.3 = "self".to_string();
                                            
                                            if let Some(_) = receiver.reference {
                                                let_var.1 = true;
                                            }
                                            if let Some(_) = receiver.mutability {
                                                let_var.2 = true;
                                            }
                                            // 到底要不要把self加进去 ?
                                            if !let_var.1 && !let_var.2{ let_var.0=1 }; // self
                                            if let_var.1 && !let_var.2{ let_var.0=3 }; // &self
                                            if !let_var.1 && let_var.2{ let_var.0=1 }; // mut self
                                            if let_var.1 && let_var.2{ let_var.0=2 }; // &mut self
                                            
                                        
                                        
                                        }
                                        syn::FnArg::Typed(pattyped) => {
                                            // patident 是名字
                                            match &*pattyped.pat{
                                                Pat::Ident(patident) => {
                                                    let_var.3 = (String::from(format!("{}", patident.ident)));
                                                }
                                                _ => {}
                                            } 
                                            match &*pattyped.ty{
                                                Type::Reference(TyRef) =>{
                                                    let_var.1 = true;
                                                    
                                                    if let Some(_mutability) = &TyRef.mutability{//mutref
                                                        let_var.0 = 2;
                                                        let_var.2 =true;
                                                    }else {//staticref
                                                        let_var.2 = false;
                                                        let_var.0 = 3;
                                                    }
                                                }
                                                Type::Path(typa) => {
                                                    let_var.0 = 1;
                                                    let_var.1 = false;
                                                    let_var.2 = false;

                                                }
                                                _ =>{}
                                            }
                                        }
                                        
                                    }
                                    graph.push_node(&(let_var.0,let_var.1,let_var.2), &let_var.3);
                                } 

                                // signature

                            }
                            for stmt in &itemMethod.block.stmts {    
                                // 从let 开始加入所有语句 主要是搜索所有的let语句，找到所有的定义变量，再进行加边
                                // 先把所有的出现的名称加入varmap中
                                alias_name_stmt(stmt, &mut graph, &mut var_map,return_value_map);
                                
                            }
                        }
                        
                        _ => ()
                    }
                    
                }
            }

            _ => () 
        }
    }



    graph
}






pub fn create_alias_hashmap(path_name: &str, funcname: &str) ->HashMap<String,(i32,bool,bool)>{
    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);

    let ast = syn::parse_file(&content).expect("ast failed");
    // 需要一个邻接表图表示？还是一个简单的并查集？
    // 最后返回一个hashmap?


    // 优先扫一遍 获取所有function/method/methodlist/返回值
    // 然后获得一个返回值map
    // 这里的methodmap是method_name/func_name + (self,return_value)
    let mut return_value_map = method_call_names(path_name);

    // 获取返回值信息了
    // 对于call的返回值 如何生成
    let graph = alias_map_genarate(&ast,funcname,&return_value_map);
    
    // 解析graph 生成hashmap
    

    let mut Varmap = HashMap::new();
    for i in 0..graph.len_num() {
        if let Some(stmtnode) = &graph.heads_list[i].data.stmt {
            match  stmtnode {
                // 新建加入节点
                stmt_node_type::Owner(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        let mut mutabe = false;
                        if varinfo.Mutability{
                            mutabe = true;
                        }
                        Varmap.insert(a.to_string(), (1 as i32, false, mutabe));
                    }
                    
                }
                stmt_node_type::MutRef(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        Varmap.insert(a.to_string(), (2 as i32, varinfo.Reference, varinfo.Mutability));
                    }
                }
                stmt_node_type::StaticRef(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        Varmap.insert(a.to_string(), (3 as i32, varinfo.Reference, varinfo.Mutability));
                    }
                }
                stmt_node_type::PTR(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        Varmap.insert(a.to_string(), (10 as i32, varinfo.Reference, varinfo.Mutability));
                    }
                }
                _ => ()
                
            }
        }

    }

    Varmap
}


#[test]
fn test_create_alias_hashmap()
{
    let path_name = "./src/parse/tests/1.rs";
    let mut file = File::open(Path::new(path_name))
        .expect("Open file failed");
    let name = "main";
    
    let mut content = String::new();
    file.read_to_string(&mut content);

    
    let ast = syn::parse_file(&content).expect("ast failed");

    // 需要一个邻接表图表示？还是一个简单的并查集？
    // 最后返回一个hashmap?
    
    let mut return_value_map = method_call_names(path_name);
    
    // 获取返回值信息了
    // 对于call的返回值 如何生成
    let graph = alias_map_genarate(&ast,name, &return_value_map);
    
    graph.show();

    // 根据图 创建所需要的别名表

}

