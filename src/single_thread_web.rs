use std::{
    fs,
    io::{prelude::*,BufReader},
    net::{TcpListener,TcpStream},
    thread,
    time::Duration,
};

use bytes::{Buf, buf};
use std::{sync::mpsc};
use std::sync::Arc;
use std::sync::Mutex;





// struct Job;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message{
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected;shutting down.");
                    break;
                }
            }
        });
        Worker {id,thread:Some(thread)}
    }
}

pub struct ThreadPool{
    sender: Option<mpsc::Sender<Job>>,
    workers:Vec<Worker>
}


impl ThreadPool{
    pub fn new(size:usize) -> ThreadPool{
        assert!(size > 0);

        let (sender,receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers = Vec::with_capacity(size);
        
        for id in 0..size{
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }

        ThreadPool { 
                    workers,
                    sender:Some(sender)
            }

    }

    pub fn execute<F>(&self,f:F)
    where F:FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}


impl Drop for ThreadPool{
    fn drop(&mut self){
        drop(self.sender.take());

        for worker in &mut self.workers{
            println!("Shuting down worker {}",worker.id);

            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}



fn handle_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    // let http_request:Vec<_> = buf_reader
    //                                 .lines()
    //                                 .map(|result| result.unwrap())
    //                                 .take_while(|line| !line.is_empty())
    //                                 .collect();
    // println!("Request:{:#?}",http_request);

    let request_line = buf_reader.lines().next().unwrap().unwrap();    //读取第一行
    println!("first line:{}",request_line);

    let (status_line,filename) = match &request_line[..]{
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "src/hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND","src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();


    
}

pub fn thread_pool_web(){

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2){
        let stream = stream.unwrap();
        println!("Connection established");
        
        pool.execute(|| {
                handle_connection(stream);            
        });
    }
}






















pub fn test_thread_pool_web(){

    thread_pool_web();
}