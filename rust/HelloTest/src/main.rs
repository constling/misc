mod shape;
mod util;

use futures::channel::mpsc::Receiver;
use futures::executor::block_on;
use futures::StreamExt;
use shape::signal;
use std::any::{type_name, TypeId};
use std::process::id;
use std::time::Duration;
use time::macros::{offset, time};
use time::OffsetDateTime;
use tracing::{event, info, Instrument, Level};
use serde::{Deserialize, Serialize};
use expression_engine::{create_context, execute, Value};

async fn func() {
    println!("good man");
    event!(Level::INFO, "this is info runc");
}

fn testSignal(tx: drain::Signal) {
    println!("i come 1");
    block_on(tx.drain());
    println!("i come 2");
}

fn testTime() {
    println!("{}", OffsetDateTime::now_utc());
    println!(
        "{}",
        OffsetDateTime::now_utc().to_offset(time::macros::offset!(+8))
    );
    println!(
        "{}",
        OffsetDateTime::now_utc()
            .to_offset(time::macros::offset!(+8))
            .replace_time(time!(00:00))
    );
}

fn testMQ() {
    let addr = std::env::var("AMQP_ADDR")
        .unwrap_or_else(|_| "amqp://guest:guest@127.0.0.1:5672/%2f".into());
    println!("{}", addr);

    tracing::info!("Connecting to AMQP server");

    block_on(async {
        let conn = lapin::Connection::connect(&addr, lapin::ConnectionProperties::default())
            .await
            .expect("connection error");

        info!("CONNECTED");

        let channel = conn.create_channel().await.expect("create_channel");
        info!(state=?conn.status().state());

        channel.basic_publish("", "event_log", Default::default(), "1234".to_string().as_bytes(), Default::default()).await;
        //创建队列
        /*let queue = channel
        .queue_declare(
            "testchannel",
            lapin::options::QueueDeclareOptions::default(),
            Default::default(),
        )
        .await
        .expect("queue_declare");*/

        /*info!("will consume");
        let mut consumer = channel
            .basic_consume(
                "demochannel",
                "my_consumer",
                lapin::options::BasicConsumeOptions::default(),
                Default::default(),
            )
            .await
            .expect("basic_consume");

        info!(state=?conn.status().state());

        while let Some(delivery) = consumer.next().await {
            info!(message=?delivery, "received message");
            if let Ok(delivery) = delivery {
                delivery.ack(Default::default()).await.expect("basic_ack");
            }
        }*/
    });
}

fn test() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn testSome() {
    let x = Some(3);
    if let Some(4) = x {
        println!("good");
    }

    if let Some(3) = x {
        println!("good2");
    }

    if let Some(y) = x {
        println!("{}", y);
    }
}

fn testvector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn testString() {
    let data = "initial contents";

    let s1 = data.to_string();

    // 该方法也可直接用于字符串字面量：
    let s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");

    let mut s4 = String::from("foo");
    s4.push_str("bar");
}

fn testString2() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('a');
    println!("s2 is {}", s2);

    let mut s3 = String::from("a");
    s3.push_str(s2);
    println!("s3 is {}", s3);
}

fn testString3() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn testfile() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // unwrap抛出panic
    //let f = File::open("hello.txt").unwrap();
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 错误向上传播
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // f.read_to_string(&mut s)?;  Ok(s); 如下同义
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

use std::error::Error;

fn testOpen() -> Result<(), Box<dyn Error>> {
    // ? 可返回Result类型
    let f = File::open("hello.txt")?;

    Ok(())
}

fn testclose() {
    use std::thread;
    use std::time::Duration;

    // 定义闭包，num为参数
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(5);
}

// T是fn trait的闭包类型
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

// box定义递归
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn testbox() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}


use std::thread;

// std::sync::mpsc::channel会阻塞


pub enum Message {
    /// List event log since a given time.
    ListEventLogSince,

}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // 如果 `current_age` 是 `None`，这将返回 `None`。
    // 如果 `current_age` 是 `Some`，内部的 `u8` 将赋值给 `next_age`。
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age))
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Reason {
    V1,
    V2
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    reason:Reason
}

fn textJson() {

    let point = Point{x:1, y:2, reason:Reason::V1};

    let json: String = serde_json::to_string(&point).unwrap();
    println!("{}", json);
}

use tokio::runtime::Builder;
use cel_interpreter::{Context, Program};
use std::thread::Thread;
use rust_decimal_macros::dec;
#[tokio::main]
async fn main() {
    // 环境变量
    dotenvy::dotenv().ok();
    // 日志初始化
    tracing_subscriber::fmt::init();

    let mut ctx = create_context!(
        "user_id"=>"1",
        "from_id"=>"2"
    );

    
    ctx.set_variable("today_record", Value::Number(dec!(10.to_string())));

    let rule = "(today_record > 2) ?  true : false";
    let Ok(res) = execute(&rule, ctx) else {
        return;
    };

    let match_flag = match res {
        Value::Bool(item)=>item,
        Value::Number(item) => {
            true
        }
        _=>false
    };

    //textJson();
    util::testRhai();

    //testMQ();
    let num = 5;
    //println!("{}", Reason::V1);
    
    let runtime = Builder::new_multi_thread().worker_threads(1).build();

    let (mut tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
   

    tokio::spawn(async move {        
        println!("good 3");
        println!("Child thread ID is: {:?}", thread::current().id());
        thread::sleep(Duration::from_secs(2));
        tx.send("hello");
        println!("good 4");
    });
    
    tokio::spawn(async move {
        println!("good 1");
        let msg = rx.recv().await.unwrap();
        println!("good 2");
        println!("Child thread ID is: {:?}", thread::current().id());
    });

    
    signal::shutdown().await;
    //let (drain_tx, drain_rx) = drain::channel();

    /*let rx2 = drain_rx.clone();
    tokio::spawn(async move {
        rx2.signaled();
        drain_rx.clone().signaled();
        println!("i come here3");
    });*/

    //testSignal(drain_tx);

    // info!("this is info log") 与之等价, debug!, error! ... 亦然
    //event!(Level::INFO, "this is info log");
    //event!(Level::INFO, "this is info log2");
    //util::testDrain();
    //util::testmpscChannel();

    //signal::shutdown().await;
    /*let wait2 = tokio::spawn(
        func().instrument(tracing::info_span!("reputation").or_current())
    );

    wait2.await.unwrap();
    thread::sleep(Duration::from_secs(3));*/
}

/* func insertUserDemo()(err error){
	sqlStr := "INSERT INTO user (name,age) VALUES (:name,:age)"
	_, err = db.NamedExec(sqlStr,
		map[string]interface{}{
			"name": "七米",
			"age": 28,
		})
	return
} */
