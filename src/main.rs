use std::{
    pin::Pin,
    task::{Context, Poll},
};

use async_stream::stream;
use futures_core::Stream;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let mut pinned = Pinned {
        x: 1,
        _pin: std::marker::PhantomPinned,
    };
    let unpin = Unpin { y: &mut pinned };
    // foo関数はUnpin traitを要求するが、Unpinはムーブ可能でUnpin traitを実装しているのでエラーにならない
    f(unpin);

    // Pinned型はUnpin traitを実装していないのでエラーになる
    // f(pinned);

    // StreamExt::next()はUnpin traitを要求するので、pin_mut!マクロを使ってピン止めする
    // ピン止めするとUnpin traitを実装する
    let stream = stream().await;
    futures_util::pin_mut!(stream);
    
    while let Some(item) = stream.next().await {
        println!("{}", item);
    }

    let mut stream = MyStream();
    // std::pin::pin!マクロを使ってピン止めする場合
    let mut stream = std::pin::pin!(stream);

    if let Some(item) = stream.next().await {
        println!("{}", item);
    }
}

async fn stream() -> impl Stream<Item = i32> {
    stream! {
        yield 1;
    }
}

struct MyStream();

impl Stream for MyStream {
    type Item = i32;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(42))
    }
}

// Unpin traitを実装しない型
#[derive(Debug)]
struct Pinned {
    x: i32,
    _pin: std::marker::PhantomPinned,
}

// Unpin traitを実装する型への参照を持つ型
// 参照自体はつねにムーブ可能なのでこの型自体はUnpinを実装する
#[derive(Debug)]
struct Unpin<'a> {
    y: &'a mut Pinned,
}

fn f<T>(x: T)
where
    T: std::marker::Unpin + std::fmt::Debug,
{
    println!("{:?}", x);
}
