// 创建一个邻接表 存储整个控制流图
// 本想支持泛型，做一个完整的邻接表，但是发现如果邻接表表头用vec就不能泛型，但是不用vec就不能顺序存储直接查找会掉很多效率
// 只能固定邻接表表头类型了，只有邻接表表头保存了数据，其他的边都是表示路径没有任何数据

use syn::Signature;

use super::parse_var::{node, VarInfo,stmt_node_type, block_node_type, FuncInfo};

use super::parse_var;

// // 邻接表内节点
// pub struct adj_node<T>{
//     // 内容
//     t:T,
//     // 节点编号
//     node_num: i32,

//     // 可以加入其他内容，但是不需要
// }

// 邻接表头 头只表示相当于序号？

// 边界点，单纯用来保存点相连的边，不存储其他信息

#[derive(Clone,Debug)]
pub struct edge_node{
    head_num:usize,
    nxt: Option<Box<edge_node>>,
}

impl  edge_node {
    // 薪节点
    pub fn new(i: usize)-> edge_node{
        edge_node{
            head_num: i,
            nxt: None,
        }
    }
    pub fn get_num(&self) -> usize{
        self.head_num
    }
}
#[derive(Clone,Debug)]
pub struct adj_node{
    // 节点编号
    head_num: i32,
    // node 是真正的节点信息，包括节点是不是fn函数/ 是不是mixed等等
    pub data: node,

    // 下一个节点
    nxt: Option<Box<edge_node>>,
    // head 保留一个尾部节点
    // last_node: &'a mut Option<Box<adj_node<T>>>,
}


impl adj_node{
    // 创建一个新的节点
    // 暂时不考虑 有一个尾部节点优化速度 实际上除了match这种大量分支的，不会有非常多的边影响效率
    // 新建表头节点需要node数据填充，需要注意
    pub fn new(i: i32, data: node)-> adj_node{
        adj_node { head_num: i, data: data ,nxt: None, }
    }
    // 设置本节点编号
    pub fn set_num(&mut self, i: i32){
        self.head_num = i;
    }

}


// 只有head节点中的保存数据，后面的next形成的链表节点中的data并无意义 单纯用来存储边关系
#[derive(Clone,Debug)]
pub struct Adjlist{
    pub heads_list: Vec<adj_node>,
    // tail_list: Vec<>
}

impl Adjlist{
    // new 1个头
    pub fn new()->Adjlist{
        Adjlist { heads_list: Vec::<adj_node>::new() }
    }
    // push 新建一个节点进去 可能不太正确
    
    pub fn push(&mut self, data: node){
        //  是否在新建之前就需要 setnum？
        let mut new_node = adj_node::new(0,data);
        new_node.set_num(self.heads_list.len() as i32);
        self.heads_list.push(new_node);
        
    }

    // 新建一条u到v的边
    pub fn add(&mut self, u: usize, v:usize){
        if u >= self.heads_list.len() ||  v >= self.heads_list.len(){
            println!("add : not exist u or v");
        } 
        let head = &mut self.heads_list[u];
        // 新节点
        let new_node = Box::new(edge_node::new(v));
        
        if let Some(current) = &mut head.nxt{
            // 如果已经存在边，那么继续向下遍历 直到结尾 
            
            let mut cur = current.as_mut();
            while !(cur.nxt.is_none()){
                cur = cur.nxt.as_mut().unwrap();
            }
 
            cur.nxt = Some(new_node);

        }else { //后面没有边 创建一个节点直接塞进去

            head.nxt = Some(new_node);
        }
    }
    // 1 代表owner； 2代表mut ref ；3代表immutref；接下来是 ref 和 mutability
    pub fn push_node(&mut self ,statement: &(i32, bool, bool), name: &String) {
        let var = VarInfo{Name: Some(name.to_string()), Reference: statement.1, Mutability: statement.2, Number: self.len_num() }; 
        match statement.0 {
            1 =>{
                self.push(node { 
                    stmt:Some(stmt_node_type::Owner(var)), 
                    block: None 
                });
            }
            2 =>{
                self.push(node { 
                    stmt:Some(stmt_node_type::MutRef(var)), 
                    block: None 
                });
                // println!("wrong");
            }
            3 => {
                self.push(node { 
                    stmt:Some(stmt_node_type::StaticRef(var)), 
                    block: None 
                });
                // println!("wrong");
            }
            10 => {
                self.push(node { 
                    stmt:Some(stmt_node_type::PTR(var)), 
                    block: None 
                });
            }
            _=>{}
        }
        
    } 
    // func call
    pub fn push_func_node(&mut self , name: &String, start:bool, end:bool, return_value: usize) {
        self.push(node { 
            stmt:Some(
                stmt_node_type::Function(
                    FuncInfo{
                        Name: Some(name.to_string()), 
                        return_value: return_value, 
                        Number: self.len_num(), 
                        Start:start, 
                        End:end,
                        method_call: -1,
                    })
                ) , 
            block: None 
        });
    }
    // methodcall
    pub fn push_method_node(&mut self , name: &String, start:bool, end:bool, method_self: i32, return_value: usize) {
        self.push(node { 
            stmt:Some(
                stmt_node_type::Function(
                    FuncInfo{
                        Name: Some(name.to_string()), 
                        return_value: return_value, 
                        Number: self.len_num(), 
                        Start:start, 
                        End:end,
                        method_call: method_self,
                    })
                ) , 
            block: None 
        });
    }


    pub fn push_block_start(&mut self) {
        self.push(node { 
            stmt: None, 
            block: Some(block_node_type::BLOCK_START) 
        });
    }

    pub fn push_block_end(&mut self) {
        self.push(node { 
            stmt: None, 
            block: Some(block_node_type::BLOCK_END) 
        });
    }

    pub fn push_block_none(&mut self) {
        self.push(node { 
            stmt: None, 
            block: Some(block_node_type::BLOCK_NONE) 
        });
    }

    // delete 删除一条u到v的边 在图构造时使用
    
    pub fn delete(&mut self, u: usize, v:usize){

        if u >= self.heads_list.len() ||  v >= self.heads_list.len(){
            println!("delete: not exist u or v");
        } 

        let head = &mut self.heads_list[u];

        if head.nxt.is_none(){
            println!("delete: not exsit {} to {} ", u,v);
            return;
        }

        // 直接解决掉当前节点
        let cur_num =head.nxt.as_mut().unwrap().get_num(); 
        // 如果当前节点下一个就是 ，否则向下遍历
        if cur_num == v{
            head.nxt= head.nxt.as_mut().unwrap().nxt.clone();
        }else {
            if let Some(current) = &mut head.nxt{
                let mut cur = current.as_mut();
                while !(cur.nxt.is_none()){
                    // 找到v
                    if cur.nxt.as_mut().unwrap().get_num() == v {
                        cur.nxt = cur.nxt.as_mut().unwrap().nxt.clone();
                        println!("found {} to {} and delete", u,v);
                        break;
                    }

                    cur = cur.nxt.as_mut().unwrap();
                }

            }else {
                println!("not exsit {} to {} ", u,v);
            }
        }


    }

    // 打印整个邻接表
    pub fn show(&self){
        
        let mut i =0;
        loop{

            let head = &self.heads_list[i];
            print!("{}",i);

            if let Some(a) = &head.data.stmt{
                match a {
                    stmt_node_type::Owner(info) => {
                        if(info.Mutability){
                            print!("Mut Owner: {:?}",info.Name);
                            print!("  ");
                        }else{
                            print!("Imut Owner: {:?}",info.Name);
                            print!("  ");
                        }
                        

                    }
                    stmt_node_type::MutRef(info) => {
                        print!("Mut Ref: {:?}",info.Name);
                        print!("  ");
                    }
                    stmt_node_type::StaticRef(info) => {
                        print!("Imut Ref: {:?}",info.Name);
                        print!("  ");
                    }
                    stmt_node_type::Function(info) => {
                        
                        print!("Function: {:?}",info.Name);
                        
                        if info.Start{
                            print!("Start");
                        }else {
                            print!("End");
                        }
                        print!("  ");
                    }
                    stmt_node_type::PTR(info) => {
                        print!("PTR : {:?}",info.Name);
                    }
                }
            }
            if let Some(b) = &head.data.block{
                match b{
                    block_node_type::BLOCK_START => {
                        print!(" {{ ");
                        print!("  ");
                    }
                    block_node_type::BLOCK_END => {
                        print!(" }} ");
                        print!("  ");
                    }
                    block_node_type::BLOCK_NONE => {
                        print!(" 🔚 ");
                        print!("  ");
                    }
                }
            }
            print!("边连接 edges::");
            if let Some(current) = & head.nxt{
                print!("{} ", current.as_ref().get_num());
                let mut cur = current.as_ref();
                while !(cur.nxt.is_none()){
                    print!("{} ",cur.nxt.as_ref().unwrap().get_num());
                    cur = cur.nxt.as_ref().unwrap();
                }
            }
            println!();
            i = i+1 ;
            if i>=self.heads_list.len(){
                break;
            }
        }
    }
    pub fn len_num(&self)-> usize{
        self.heads_list.len()
    }

    // 返回节点acfg信息: x ((attribute1, a2,a3,a4, a5, a6 ,a7),(,,,,,)    )
    pub fn vector_x_attribute(&self, i: usize) -> (String, i32, i32, i32, String, i32, i32,i32 ,i32) { // 返回节点为i的attribute 等 
        
        // 前四个 0name/1type/2mute/3ref 后两个 4funcname/5funcscope 后一个6methodcall self 最后7returnvalue /最后 8block
        let mut x = ("-1".to_string(),-1,-1,-1,"-1".to_string(),-1, -1, -1, -1);
        // 根据data开始 计算attribute
        // x 的attribute是否 不需要string？ 考虑删除string
        if let Some(stmt) = &self.heads_list[i].data.stmt{
            match  stmt {
                // 新建加入节点
                stmt_node_type::Owner(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        x.0 = a.to_string();
                        x.1 = 1;
                        x.2 = varinfo.Mutability as i32;
                        x.3 = varinfo.Reference as i32;
                    }
                    
                }
                stmt_node_type::MutRef(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        x.0 = a.to_string();
                        x.1 = 2;
                        x.2 = varinfo.Mutability as i32;
                        x.3 = varinfo.Reference as i32;
                    }
                }
                stmt_node_type::StaticRef(varinfo) => {
                    if let Some(a) = &varinfo.Name {
                        x.0 = a.to_string();
                        x.1 = 3;
                        x.2 = varinfo.Mutability as i32;
                        x.3 = varinfo.Reference as i32;
                    }
                }
                // 包括 method call
                stmt_node_type::Function(funcinfo) => {
                    if let Some(a) = &funcinfo.Name {
                        x.4 = a.to_string();
                        if funcinfo.Start {
                            x.6 = 0;
                        }else if funcinfo.End{
                            x.6 = 1;
                        }
                    }
                    x.5 =funcinfo.method_call;
                    x.7 =funcinfo.return_value as i32;
                }
                stmt_node_type::PTR(info) => {
                    // wu
                }
                
            }
        }
        if let Some(block) = &self.heads_list[i].data.block{
            match block{
                block_node_type::BLOCK_START=>{
                    x.8 = 0;
                }
                block_node_type::BLOCK_END=>{
                    x.8 = 1;
                }
                block_node_type::BLOCK_NONE=>{
                    x.8 = 2;
                }
            }
        }
        return x;

    }
    pub fn vector_edge_attribute(&self,i: usize) ->Vec<usize> { // 返回节点为i的所有边？
        let mut edge = Vec::new();

        let head = &self.heads_list[i];
        if let Some(current) = & head.nxt{
            edge.push(current.as_ref().get_num());

            let mut cur = current.as_ref();
            while !(cur.nxt.is_none()){
                // get_num 既边的序号  
                edge.push(cur.nxt.as_ref().unwrap().get_num());
                cur = cur.nxt.as_ref().unwrap();
            }
        }

    
        edge // 当前节点的所有边
        
    }

}

// 针对邻接表进行测试
#[test]

fn test_adjlist(){
    let mut a = Adjlist::new();

    // a.push(node::new()); 
    // a.push(node::new());
    // a.push(node::new());
    // a.push(node::new());

    // // 只有第一个点add进去了？

    // a.add(0, 1);

    // a.add(0, 2);

    // a.add(1, 2);
    // a.add(2, 3);
    // a.add(3, 1);


    // a.show();

    // a.delete(0, 1);
    
    // a.delete(0, 3);
    // a.show();

}
