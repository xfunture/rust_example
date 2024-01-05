
use serde::{Serialize,Deserialize};
use std::num::ParseIntError;
use std::result::Result::Ok;

use tokio::task::yield_now;
use std::rc::Rc;

#[derive(Debug)]
pub enum Myerror{
    Onerror,
    Seconderror,
    Thirderror
}

mod checked {
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }


    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    // 中间函数
    pub fn op_(x: f64, y: f64) -> MathResult {
        // 如果 `div` “失败” 了，那么返回 `DivisionByZero`
        let ratio = div(x, y)?;

        // 如果 `ln` “失败” 了，那么返回 `NegativeLogarithm`
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Err(why) => panic!("{}",match why {
                MathError::NegativeLogarithm
                    => "logarithm of negative number",
                MathError::DivisionByZero
                    => "division by zero",
                MathError::NegativeSquareRoot
                    => "square root of negative number",
            }),
            Ok(value) => println!("{}", value),
        }
    }
}


pub fn div(x:f64,y:f64) -> Result<f64,String>{

    if y == 0.0{
        Err("error".to_string())
    }else{
        Ok(x / y)
    }
}


pub fn test_question_mark_operator() ->Result<f64,String>{
    // let result = checked::op(10.0, 1.0);
    
    //使用unwrap可以取出Result 中的值f64
    let result = checked::div(10.0,1.0).unwrap();
    println!("result:{:?}",result);

    //在不使用unwrap 的情况下，返回的是Ok(f64)
    let result = checked::div(10.0,1.0);
    println!("result:{:?}",result);

    let result = div(10.0,1.0)?;
    println!("result:{:?}",result);

    Ok(1.0/2.0)

}


// 测试unwrap
fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}


// 测试match
fn multiply_v1(first_number_str:&str,second_number_str: &str) -> Result<i32,ParseIntError>{
    match first_number_str.parse::<i32>(){
        Ok(first_number) => {
            match second_number_str.parse::<i32>(){
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}


// 测试Result map
fn multiply_v2(first_number_str: &str,second_number_str: &str) -> Result<i32,ParseIntError>{
    first_number_str.parse::<i32>().and_then(|first_number|{
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

// 测试问号运算符
fn multiply_v3(first_number_str:&str,second_number_str:&str) -> Result<i32,ParseIntError>{
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}


fn print(result:Result<i32,ParseIntError>){
    match result{
        Ok(n) => println!("n is {}",n),
        Err(e) => println!("Error:{}",e),
    }
}

pub fn test_unwrap(){
    // let twenty = multiply("10", "2");
    // println!("double is {}", twenty);

    // let tt = multiply("t", "2");
    // println!("double is {}", tt);

    let twenty = multiply_v1("10", "2");
    print(twenty);

    let tt = multiply_v1("t", "2");
    print(tt);

    let twenty = multiply_v2("10", "2");
    print(twenty);

    let tt = multiply_v2("t", "2");
    print(tt);

    let twenty = multiply_v3("10", "2");
    print(twenty);

    // let tt = multiply_v3("t", "2");
    // print(tt);


}




