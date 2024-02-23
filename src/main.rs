use tokio::{self,runtime::Runtime,time::{self,Duration}};
use tokio::sync::broadcast::{self,Sender};
use tokio::task::JoinSet;
use chrono::Local;
use anyhow::Result;
async fn sleep(n:u64) -> u64{
    time::sleep(Duration::from_secs(n)).await;
    n
}

fn now() -> String {
    Local::now().format("%F %T").to_string()
}



// #[tokio::main]
// async fn main(){
    // let rt = Runtime::new().unwrap();
    // rt.block_on(async{

    //     tokio::select!{
    //         v = sleep(2) => println!("sleep 2 secs,brandch 1 done: {}",v),
    //         v = sleep(3) => println!("sleep 3 secs,branch 2 done: {}",v),
    //     };
    //     println!("select! done");
    // });

    //测试select!宏
    // let mut count = 0u8;
    // println!("count:{}",count);
    // loop{
    //     tokio::select! {
    //         // 如果取消biased，挑选的任务顺序将随机，可能会导致分支中的断言失败
    //         biased;q
    //         _ = async {println!("hello1,{}",count)}, if count < 1 => { count += 1; assert_eq!(count, 1); }
    //         _ = async {println!("hello2,{}",count)}, if count < 2 => { count += 1; assert_eq!(count, 2); }
    //         _ = async {println!("hello3,{}",count)}, if count < 3 => { count += 1; assert_eq!(count, 3); }
    //         _ = async {println!("hello4,{}",count)}, if count < 4 => { count += 1; assert_eq!(count, 4); }
    //         else => { println!("end"); }
    //     };
    // }

// }


#[tokio::main]
async fn main() -> Result<()> {
    println!("hello,world");
    Ok(())
}