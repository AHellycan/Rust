//extern crate trpl;
use trpl::{Html, Either};
use std::{future::Future, time::Duration, pin::Pin};

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
    })
}

async fn page_title(url: &str) -> (&str,Option<String>) {
    let response = trpl::get(url).await;
    let response_text  = response.text().await;
    let title = Html::parse(&response_text).select_first("title").map(|title| title.inner_html());
    (url,title)
}