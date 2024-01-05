

use anyhow::{Ok,Result};
use rust_example::test::test_question_mark_operator;
use rust_example::test::test_unwrap;
use rust_example::single_thread_web::test_thread_pool_web;

//test redis
use rust_example::redis::test_redis;



#[tokio::main]
async fn main() -> Result<()> {

    // 测试问号运算符
    // let num = test_question_mark_operator()?;
    // println!("num:{:?}",num);

    // 测试unwrap,问号运算符，Result<T,Err>
    // test_unwrap();

    //测试线程池服务器
    // test_thread_pool_web();

    //启动redis服务端
    test_redis().await;


    Ok(())

}