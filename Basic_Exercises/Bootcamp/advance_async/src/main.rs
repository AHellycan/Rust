//extern crate trpl;
use trpl::{Html, Either,ReceiverStream, Stream, StreamExt};
use std::{fmt::format, future::Future, pin::{pin, Pin}, time::Duration};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async{
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) = 
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
                
            };

            println!("{url} returned first");
            match maybe_title {
                Some(title) => println!("Its page title was: '{title}'"),
                None => println!("No title found"),   
            }
    });

    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });

    trpl::run(async{
        let (tx,mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move{
            let vals = vec![String::from("hi"),String::from("from"),
                       String::from("the"),String::from("future"),
                       ];
            for val in vals{
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
            }   
        };
        
        let tx_fut = async move{
            let vals = vec![String::from("more"),String::from("messages"),
                       String::from("for"),String::from("you"),
                       ];
            for val in vals{
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
            }   
        };
        
        
        let rx_fut = async{
            while let Some(received) = rx.recv().await{
                println!("got: {received}");
            }
        };
        let futures:Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(tx_fut),Box::pin(rx_fut),Box::pin(tx1_fut)];
        trpl::join_all(futures).await;
    });

    trpl::run(async{
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));
        
        while let Some(result) = messages.next().await{
            match result {
                Ok(message) => println!("received: {message}"),
                Err(reason) => println!("Problem: {reason}"),
            }
            
        }
    });

    trpl::run(async{
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval: {count}"))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));

        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await{
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => println!("Problem: {reason}"),
            }
            
        }
        
    });
}

async fn page_title(url: &str) -> (&str,Option<String>) {
    let response = trpl::get(url).await;
    let response_text  = response.text().await;
    let title = Html::parse(&response_text).select_first("title").map(|title| title.inner_html());
    (url,title)
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e","f", "g", "h", "i", "j"];
        for (index,message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("message {}", message)) {
                eprintln!("Can not send message '{message}': {send_error}");
                break;
            } 
            
        }
    });
    
    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
         
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_error) = tx.send(count) {
                eprintln!("Can not send interval '{count}': {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}