
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;
use super::parse_var;

#[test]
fn generate_test_ast(){
    let mut file = File::open(Path::new("./src/parse/tests/1.rs"))
        .expect("Open file failed");

    let mut content = String::new();
    file.read_to_string(&mut content);
    println!("{:?}",content);
}


#[test]
fn write_all_csvs(){
    // 读取所有的数据增强文件，并且把所有生成的embeddeding都放入codei文件夹下
    // 读取codei 文件夹下内容
    // "./spider_stackoverflow/src/dataok/codei/i.rs"
    let baseadd1 ="./spider_stackoverflow/src/dataok/code".to_string();

    let baseadd2 = ".rs".to_string();

    let baseadd3 = "/name.txt".to_string();
    
    let code_nums = 71;
    let aug_nums = 20;

    for i in 0..code_nums{
        let mut a = i.to_string();

        for j in 0..aug_nums{
            let code_path = baseadd1.to_string() + &i.to_string() + "/" +&j.to_string()+ &baseadd2 ;  
            let csv_path = baseadd1.to_string() + &i.to_string() + "/" +&j.to_string();
            
            let name_path = baseadd1.to_string() + &i.to_string() + "/" +&baseadd3;

            let mut file = File::open(Path::new(&name_path))
                .expect("open name.txt failed ");
            let mut name_func = String::new();
            file.read_to_string(&mut name_func);

            parse_var::csv_creat(&code_path, &csv_path, &name_func);

            // println!("{}",csv_path);


        }

        

    }


}


// 读取所有的东西

#[test]
fn write_all_csvs2(){

    let baseadd ="./spider_stackoverflow/classification2/createnew".to_string();
    let mut classfication_ = Vec::new();
    classfication_.push("func_call");
    classfication_.push("func_call + match + return_ref");
    classfication_.push("func_call + return_ref");
    classfication_.push("loop");
    classfication_.push("match");
    classfication_.push("method_call");
    classfication_.push("method_call + func_call + return_ref");
    classfication_.push("method_call + loop");
    classfication_.push("method_call + loop + return_ref");
    classfication_.push("method_call + match + return_ref");
    classfication_.push("method_call + return_ref");
    classfication_.push("not");

    // 所有的路径
    // 根据这个 获得路径 然后读取，生成

    let mut classfication_2 = Vec::new();

    classfication_2.push("e106");
    classfication_2.push("e502");
    classfication_2.push("e382");
    classfication_2.push("e499");
    classfication_2.push("e505");
    classfication_2.push("e506");
    classfication_2.push("e507");
    classfication_2.push("e515");
    classfication_2.push("e597");
    // 第二个集合
    // 读rs
    let readrs = ".rs".to_string();
    // 读function
    let readfunction = "_1.txt".to_string();
    // 读errorvariable
    let readerrorvar = "_2.txt".to_string();

    let mut nums= 0;

    for class_1 in &classfication_{
        let a = class_1.to_string();
        // 获取每一个string
        for class_2 in &classfication_2{
            let b = class_2.to_string();
            // 获取目录
            let address_one = baseadd.to_string() +  "/" + &a.to_string() + "/" + &b.to_string();
            // 
            let path = Path::new(&address_one);
            // 如果存在就进去进行计算
            if path.exists(){
                // 
                
                for i in 1..20{
                    let mut address_rs = address_one.to_string()+ "/" + &i.to_string()+&readrs.to_string();
                    let path2 = Path::new(&address_rs);
                    if path2.exists(){
                        // 如果存在rs 计算一个csv
                        // func
                        
                        let mut address_func = address_one.to_string()+ "/" + &i.to_string()+&readfunction.to_string();
                        let mut address_vars = address_one.to_string()+ "/" + &i.to_string()+&readerrorvar.to_string();

                        let mut address_csv = address_one.to_string() + "/" +&i.to_string();
                        // println!("{:?}",address_csv);
                        // 需要读取吗 还是直接传参？
                        
                        //let mut address_vars = address_one.to_string()+ "/" + &i.to_string()+&readerrorvar.to_string();
                        let mut file = File::open(Path::new(&address_func))
                            .expect("open name.txt failed ");
                        
                        //     println!("{:?}",address_rs);
                        let mut name_func = String::new();
                        file.read_to_string(&mut name_func);
                        // println!("{:?}",name_func);
                        // if name_func == "mian".to_string(){
                        //     println!("{:?}",address_rs);
                        //     // 换成main
                        // }

                        let mut file2 = File::open(Path::new(&address_vars))
                            .expect("open name.txt failed ");
                        let mut errorvariable = String::new();
                        // 用enter分开变量
                        file2.read_to_string(&mut errorvariable);
                        let vars_vec = split_string(&errorvariable);

                        // println!("{}",address_rs);
                        // println!("{}",address_csv);
                        // println!("{}",name_func);
                        // println!("{:?}",vars_vec);

                        
                         // 传入 address_rs address_csv funcname  vars_vec 
                        parse_var::csv_creat2(&address_rs, &address_csv, &name_func, &vars_vec);

                        

                        // 传给parse_var用
                        nums = nums +1;
                    }

                }



            }


        }
    }

    // 计算coreset数量

    // 计算augment数量


    // println!("{}",nums);
}

fn split_string(input: &String) -> Vec<String> {
    let mut lines = Vec::new();
    for line in input.lines() {
        lines.push(line.to_string());
    }
    lines
}


#[test]
fn write_all_csvs3(){
    // 读取所有的数据增强文件，并且把所有生成的embeddeding都放入codei文件夹下
    // 读取codei 文件夹下内容
    // "./spider_stackoverflow/src/dataok/codei/i.rs"
    let baseadd ="./spider_stackoverflow/classification2/reddit".to_string();
    let mut classfication_ = Vec::new();
    classfication_.push("func_call");
    classfication_.push("func_call + match + return_ref");
    classfication_.push("func_call + return_ref");
    classfication_.push("loop");
    classfication_.push("match");
    classfication_.push("match + return_ref");
    classfication_.push("method_call");
    classfication_.push("method_call + func_call");
    classfication_.push("method_call + func_call + return_ref");
    classfication_.push("method_call + loop");
    classfication_.push("method_call + loop + return_ref");
    classfication_.push("method_call + match + return_ref");
    classfication_.push("method_call + return_ref");
    classfication_.push("not");

    // 所有的路径
    // 根据这个 获得路径 然后读取，生成
    
    let mut classfication_2 = Vec::new();

    let mut hash_nums: HashMap<String, f32> = HashMap::new();



    classfication_2.push("e502");
    classfication_2.push("e382");
    classfication_2.push("e499");
    classfication_2.push("e505");
    classfication_2.push("e506");
    classfication_2.push("e507");
    classfication_2.push("e515");
    classfication_2.push("e597");
    // 第二个集合
    // 读rs
    let readrs = "wrong.rs".to_string();
    // 读function
    let readfunction = "1.txt".to_string();
    // 读errorvariable
    let readerrorvar = "2.txt".to_string();

    let mut nums= 0;


    let mut time_clock = Vec::new();


    for class_1 in &classfication_{
        let a = class_1.to_string();
        // 获取每一个string
        for class_2 in &classfication_2{
            let b = class_2.to_string();
            // 获取目录
            for i in 0..3600{

            
                let address_one = baseadd.to_string() +  "/" + &a.to_string() + "/" + &i.to_string()+ &b.to_string();
                // 这里是到了 2256e502
                let path = Path::new(&address_one);
                // 如果存在就进去进行计算
                if path.exists(){
                    // 
                    // 读取 wrong.rs 读取1.txt 2.txt
                    
                
                    let mut address_rs = address_one.to_string()+ "/" +&readrs.to_string();
                    let path2 = Path::new(&address_rs);
                    if path2.exists(){
                        // 如果存在rs 计算一个csv
                        // func
                        
                        let mut address_func = address_one.to_string()+ "/"+&readfunction.to_string();
                        let mut address_vars = address_one.to_string()+ "/" +&readerrorvar.to_string();

                        let mut address_csv = address_one.to_string() +"/";
                        // println!("{:?}",address_csv);
                        // 需要读取吗 还是直接传参？
                        // println!("{},{},{},{}",address_rs,address_func,address_vars,address_csv);
                        //let mut address_vars = address_one.to_string()+ "/" + &i.to_string()+&readerrorvar.to_string();
                        let mut file = File::open(Path::new(&address_func))
                            .expect("open name.txt failed ");
                        
                        //     println!("{:?}",address_rs);
                        let mut name_func = String::new();
                        file.read_to_string(&mut name_func);
                        // println!("{:?}",name_func);
                        // if name_func == "mian".to_string(){
                        //     println!("{:?}",address_rs);
                        //     // 换成main
                        // }

                        let mut file2 = File::open(Path::new(&address_vars))
                            .expect("open name.txt failed ");
                        let mut errorvariable = String::new();
                        // 用enter分开变量
                        file2.read_to_string(&mut errorvariable);
                        let vars_vec = split_string(&errorvariable);

                        // println!("{}",address_rs);
                        // println!("{}",address_csv);
                        // println!("{}",name_func);
                        // println!("{:?}",vars_vec);


                        // 传入 address_rs address_csv funcname  vars_vec 

                        let time_start = Instant::now();

                        

                        println!("{} {}",i,b);
                        parse_var::csv_creat2(&address_rs, &address_csv, &name_func, &vars_vec);
                        
                        let name = i.to_string() + &b.to_string();

                        hash_nums.insert(name,(time_start.elapsed().as_micros() as f32  )/1000.0);
                        
                        time_clock.push(time_start.elapsed().as_micros());

                        // 传给parse_var用
                        nums = nums +1;
                    }

                



                }
            }


        }
    }
    
    // println!("{:?}",time_clock);

    let mut max_time:f32 = 0.0;
    let mut min_time:f32 = 100000.0;
    let mut sum_time:f32 = 0.0;


    println!("{:?}",hash_nums);

    for i in &time_clock {
        if (i.clone() as f32) > max_time {
            max_time = i.clone() as f32;
        }
        if (i.clone() as f32) < min_time {
            min_time = i.clone() as f32;
        }
        sum_time += i.clone() as f32;
    }

    // println!("max: {}",max_time/1000.0);
    // println!("min: {}",min_time/1000.0);
    // println!("sum: {}",(sum_time/1000.0)/(time_clock.len() as f32));



}


#[test]
fn write_all_csvs4(){
    // 读取所有的数据增强文件，

    let baseadd ="./spider_stackoverflow/classification2/createnew".to_string();
    let mut classfication_ = Vec::new();
    classfication_.push("func_call");
    classfication_.push("func_call + match + return_ref");
    classfication_.push("func_call + return_ref");
    classfication_.push("loop");
    classfication_.push("match");
    classfication_.push("method_call");
    classfication_.push("method_call + func_call + return_ref");
    classfication_.push("method_call + loop");
    classfication_.push("method_call + loop + return_ref");
    classfication_.push("method_call + match + return_ref");
    classfication_.push("method_call + return_ref");
    classfication_.push("not");

    // 所有的路径
    // 根据这个 获得路径 然后读取，生成

    let mut classfication_2 = Vec::new();


    let mut hash_nums = HashMap::new();

    classfication_2.push("e502");

    hash_nums.insert("e502".to_string(), (0,0));

    classfication_2.push("e382");
    hash_nums.insert("e382".to_string(), (0,0));
    classfication_2.push("e499");
    hash_nums.insert("e499".to_string(), (0,0));
    classfication_2.push("e505");
    hash_nums.insert("e505".to_string(), (0,0));
    classfication_2.push("e506");
    hash_nums.insert("e506".to_string(), (0,0));
    classfication_2.push("e507");
    hash_nums.insert("e507".to_string(), (0,0));
    classfication_2.push("e515");
    hash_nums.insert("e515".to_string(), (0,0));
    classfication_2.push("e597");
    hash_nums.insert("e597".to_string(), (0,0));
    // 第二个集合
    // 读rs
    let readrs = ".rs".to_string();
    // 读function
    let readfunction = "_1.txt".to_string();
    // 读errorvariable
    let readerrorvar = "_2.txt".to_string();

    let mut nums= 0;

    for class_1 in &classfication_{
        let a = class_1.to_string();
        // 获取每一个string
        for class_2 in &classfication_2{
            let b = class_2.to_string();
            // 获取目录
            let address_one = baseadd.to_string() +  "/" + &a.to_string() + "/" + &b.to_string();
            // 
            let path = Path::new(&address_one);
            // 如果存在就进去进行计算
            if path.exists(){
                // 
                
                for i in 1..20{
                    let mut address_rs = address_one.to_string()+ "/" + &i.to_string()+&readrs.to_string();
                    let path2 = Path::new(&address_rs);
                    if path2.exists(){
                        // 如果存在rs 计算一个csv
                        // func
                        
                        
                        if let Some(a) = hash_nums.get_mut(&b){
                            a.0 = a.0 + 1;
                        }

                        

                        let mut address_func = address_one.to_string()+ "/" + &i.to_string()+&readfunction.to_string();
                        let mut address_vars = address_one.to_string()+ "/" + &i.to_string()+&readerrorvar.to_string();
                        // 获取func 和vars后对每个augment做一个x edge
                        let mut address_csv = address_one.to_string() + "/" +&i.to_string();


                        


                        // println!("{:?}",address_csv);
                        // 需要读取吗 还是直接传参？
                        
                        //let mut address_vars = address_one.to_string()+ "/" + &i.to_string()+&readerrorvar.to_string();
                        let mut file = File::open(Path::new(&address_func))
                            .expect("open name.txt failed ");
                        
                        //     println!("{:?}",address_rs);
                        let mut name_func = String::new();
                        file.read_to_string(&mut name_func);
                        // println!("{:?}",name_func);
                        // if name_func == "mian".to_string(){
                        //     println!("{:?}",address_rs);
                        //     // 换成main
                        // }

                        let mut file2 = File::open(Path::new(&address_vars))
                            .expect("open name.txt failed ");
                        let mut errorvariable = String::new();
                        // 用enter分开变量
                        file2.read_to_string(&mut errorvariable);
                        let vars_vec = split_string(&errorvariable);

                        // println!("{}",address_rs);
                        // println!("{}",address_csv);
                        // println!("{}",name_func);
                        // println!("{:?}",vars_vec);

                        

                        for aug in 1..15 {
                            let mut address_aug = address_one.to_string()+ "/" + &i.to_string()+"/"+&aug.to_string()+&readrs.to_string();
                            let mut address_aug_CSV = address_one.to_string()+ "/" + &i.to_string()+"/"+&aug.to_string();

                            let pathaug = Path::new(&address_aug);
                            if pathaug.exists() {
                                if let Some(a) = hash_nums.get_mut(&b){
                                    a.1 = a.1 + 1;
                                }
                                parse_var::csv_creat2(&address_aug, &address_aug_CSV, &name_func, &vars_vec);
                                println!("{:?}",pathaug);
                                nums = nums +1;
                            }
                        }

                         // 传入 address_rs address_csv funcname  vars_vec 
                        // parse_var::csv_creat2(&address_rs, &address_csv, &name_func, &vars_vec);


                        // 传给parse_var用
                        
                    }

                }



            }


        }
    }
    println!("{:?}",hash_nums);
    println!("{:?}",nums);

}



// 加入数据增强后的生产



#[test]
fn jisuannums(){
    // 读取所有的数据增强文件，并且把所有生成的embeddeding都放入codei文件夹下
    // 读取codei 文件夹下内容
    // "./spider_stackoverflow/src/dataok/codei/i.rs"
    let baseadd ="./spider_stackoverflow/classification2/stackoverflow".to_string();
    let mut classfication_ = Vec::new();
    classfication_.push("func_call");
    classfication_.push("func_call + match + return_ref");
    classfication_.push("func_call + return_ref");
    classfication_.push("loop");
    classfication_.push("match");
    classfication_.push("method_call");
    classfication_.push("method_call + func_call");
    classfication_.push("method_call + func_call + return_ref");
    classfication_.push("method_call + loop");
    classfication_.push("method_call + loop + return_ref");
    classfication_.push("method_call + match + return_ref");
    classfication_.push("method_call + return_ref");
    classfication_.push("not");

    // 所有的路径
    // 根据这个 获得路径 然后读取，生成

    let mut classfication_2 = Vec::new();


    let mut hash_nums = HashMap::new();

    classfication_2.push("e502");

    hash_nums.insert("e502".to_string(), (0,0));

    classfication_2.push("e382");
    hash_nums.insert("e382".to_string(), (0,0));
    classfication_2.push("e499");
    hash_nums.insert("e499".to_string(), (0,0));
    classfication_2.push("e505");
    hash_nums.insert("e505".to_string(), (0,0));
    classfication_2.push("e506");
    hash_nums.insert("e506".to_string(), (0,0));
    classfication_2.push("e507");
    hash_nums.insert("e507".to_string(), (0,0));
    classfication_2.push("e515");
    hash_nums.insert("e515".to_string(), (0,0));
    classfication_2.push("e597");
    hash_nums.insert("e597".to_string(), (0,0));




    // 第二个集合
    // 读rs
    let readrs = "wrong.rs".to_string();
    // 读function
    let readfunction = "1.txt".to_string();
    // 读errorvariable
    let readerrorvar = "2.txt".to_string();

    let readersucces = "success.txt".to_string();

    let readerfail = "failed.txt".to_string();

    let mut fnums= 0;
    let mut snums= 0;
    let mut nnums= 0;




    for class_1 in &classfication_{
        let a = class_1.to_string();
        // 获取每一个string
        for class_2 in &classfication_2{
            let b = class_2.to_string();

            // 获取e


            // 获取目录
            for i in 0..3600{

            
                let address_one = baseadd.to_string() +  "/" + &a.to_string() + "/" + &i.to_string()+ &b.to_string();
                // 这里是到了 2256e502
                let path = Path::new(&address_one);
                // 如果存在就进去进行计算
                if path.exists(){
                    // 
                    // 读取 wrong.rs 读取1.txt 2.txt
                    
                    
                    let mut address_f = address_one.to_string()+ "/" +&readerfail.to_string();
                    let mut address_s = address_one.to_string()+ "/" +&readersucces.to_string();
                    let path2 = Path::new(&address_f);
                    let path3 = Path::new(&address_s);
                    if path2.exists(){
                        fnums +=1;
                        if let Some(a) = hash_nums.get_mut(&b){
                            a.1 = a.1 + 1;
                        }
                       

                    }
                    if path3.exists(){
                        snums +=1;
                        if let Some(a) = hash_nums.get_mut(&b){
                            a.0 = a.0 + 1;
                            a.1 = a.1 + 1;
                        }
                    }
                    if !path2.exists() && !path3.exists(){

                        println!("{:?}",path);
                    }

                    


                }
            }


        }
    }
    println!("{:?}",hash_nums);
}