extern crate redis;
use futures::executor::block_on;
use redis::Commands;
use std::sync::mpsc;
use std::thread::sleep;
use std::thread::spawn;

fn do_something(con: &mut redis::Connection) -> redis::RedisResult<()> {
    let _: () = con.set("my_key", "1234")?;
    let test: String = con.get("my_key")?;
    Ok(())
}

pub fn testRedis() {
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();

    let mut conn: redis::Connection = client.get_connection().unwrap();
    // why
    //conn.set("mykey", "1234");

    do_something(&mut conn);
}

pub mod util {

    pub fn showame() {
        println!("good man")
    }
}

#[derive(Debug, Default)]
struct Song {
    name: String,
}

async fn learn_song(name: Option<String>) -> Song {
    match name {
        Some(name) => {
            println!("the song_name is {name}");
            Song { name }
        }
        None => Song::default(),
    }
}
async fn sing_song(song: Song) {
    dbg!(song);
}
async fn dance() {
    println!("i am dancing ~");
}

async fn learn_and_sing(name: Option<String>) {
    let song = learn_song(name).await;
    sing_song(song).await;
}

async fn run() {
    let song_name = Some(String::from("晴天"));
    let f1 = learn_and_sing(song_name);
    let f2 = dance();
    futures::join!(f1, f2);
}

pub fn testAsyn() {
    let song_name = Some(String::from("晴天"));
    let song = block_on(learn_song(song_name));
    block_on(sing_song(song));
    //block_on(dance());
}

pub fn testDrain() {
    let (drain_tx, drain_rx) = drain::channel();

    println!("i come");

    tokio::spawn(async move {
        println!("i come 1");
        drain_tx.drain().await;
        println!("i come 2");
    });

    let drain2 = drain_rx.clone();

    tokio::spawn(async move {
        println!("i wait you");
        
        
    });

}

pub fn testmpscChannel() {
    let (sender, receiver) = mpsc::channel();

    spawn(move || {
        for i in 0..1000 {
            sender.send(i).unwrap();
            sleep(std::time::Duration::from_nanos(10));
        }
    });

    sleep(std::time::Duration::from_millis(1000));

    loop {
        match receiver.recv_timeout(std::time::Duration::from_nanos(0)) {
            Ok(value) => {
                println!("received {}", value);
            }
            _ => {
                break;
            }
        }
    }
    println!("done");
}


///************ */
use std::time::Duration;

use tokio::select;

#[tokio::main]
async fn testSelect() {
    let (sender1, mut receiver1) = tokio::sync::mpsc::unbounded_channel::<String>();
    let (sender2, mut receiver2) = tokio::sync::mpsc::unbounded_channel::<String>();
    let (sender3, mut receiver3) = tokio::sync::mpsc::unbounded_channel::<String>();

    let (shutdown_sender, mut shutdown_receiver) = tokio::sync::watch::channel(());
    for i in 0..3 {
        sender1.send(i.to_string()).unwrap();
        sender2.send(i.to_string()).unwrap();
        sender3.send(i.to_string()).unwrap();
    }

    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        shutdown_sender.send(()).unwrap(); //两秒后关闭
    });

    loop {
        select! {
            ret = receiver1.recv() => {
                println!("channel 1 received: {:?}", ret);
            },
            ret = receiver2.recv() => {
                println!("channel 2 received: {:?}", ret);
            },
            ret = receiver3.recv() => {
                println!("channel 3 received: {:?}", ret);
            },
            _ = shutdown_receiver.changed() => {
                println!("shutdown received");
                break;
            }
        };
    }
}


use rhai::{Engine, Scope, EvalAltResult};
pub fn testRhai() {
    let engine = Engine::new();

    let mut scope = Scope::new();

    scope.push("x", 42_i64).push("y", 999_i64);
    let Ok(result) = engine.eval_with_scope::<i64>(&mut scope, "x + y") else {
        return;
    };

    println!("good man");
}