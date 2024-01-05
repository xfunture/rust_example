
use futures::TryFutureExt;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use bytes::Bytes;
use mini_redis::client;

// use mini_redis::Command::{self,Get,Set};



// mpsc 多生产者，单消费者模式
// oneshot 单生产者，单消费者，一次只能发送一条消息
// broadcast 多生产者，多消费者，其中每一条消息都可以被所有接受者收到，因此是广播
// watch 单生产者，多消费者，只保存一条最新的消息，因此接受者只能看到最近的一条消息，例如，这种模式适用于配置文件变化的监听
// async--channel 多生产者、多消费者，且每一条消息只能被其中一个消费者接收。




#[derive(Debug)]
enum Command{
    Get{
        key:String,
        resp:Responder<Option<Bytes>>,
    },
    Set{
        key:String,
        val:Bytes,
        resp:Responder<Option<Bytes>>,
    }
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;


#[tokio::main]
async fn main(){

    //mpsc: 多生产者，单消费者
    let (tx,mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();


    //消费者
    //将消息通道的接受者rx 的所有权转移到管理任务中
    let manager = tokio::spawn(async move {



            //Establish a connection to the server
            //建立redis服务器的连接
            let mut client = client::connect("127.0.0.1:6379").await.unwrap();


            // 开始接收消息
            while let Some(cmd) = rx.recv().await {
                use Command::*;
                match cmd{
                    Get {key,resp} => {
                        let res = client.get(&key).await;
                        // let _ = resp.send(res);
                        // println!("\nGet function:\nkey:{}\nvalue:{:#?}",key,res.unwrap());

                    }
                    Set {key,val,resp} => {
                        let res = client.set(&key,val).await.unwrap();
                        // let _ = resp.send(res);
                        // println!("\nSet function:\nkey:{}\nvalue:{:?}",key,res);


                    }
                }
            }
    });

    //生产者
    //生成两个任务，一个用于获取key,一个用于设置key
    let t1 = tokio::spawn(async move {
        let (resp_tx,resp_rx)  = oneshot::channel();

        let cmd = Command::Get{
                key:"foo".to_string(),
                resp:resp_tx,
        };

        //发送Get请求
        tx.send(cmd).await.unwrap();

        //等待恢复
        let res = resp_rx.await;
        println!("\noneshot GOT = {:?}",res);
    });

    let t2= tokio::spawn(async move{
        let (resp_tx,resp_rx)  = oneshot::channel();
        let cmd = Command::Set{
            key: "foo".to_string(),
            val: "bar".into(),
            resp:resp_tx,
        };

        //发送Get请求
        tx2.send(cmd).await.unwrap();

        //等待回复
        let res = resp_rx.await;
        println!("\noneshot GOT = {:?}",res);

    });

    //我们让3个任务，按照需要的顺序开始运行
    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

}
