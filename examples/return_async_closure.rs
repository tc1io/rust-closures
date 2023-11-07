#![feature(impl_trait_in_fn_trait_return)]
//#![feature(async_closure)]
use std::future::Future;
use rust_closures::*;

fn make_closure_async(db: DBAsync) -> impl Fn(i32) -> impl Future<Output=Item> {
    // move |id:i32| {
    //     let c = db.clone();
    //     async move {
    //         c.get_item_async(id).await
    //     }
    // }

    move |id:i32| {
        let mut c = db.clone();
        async move {
            c.put_item_async(id,Item::from("my item")).await;
            c.get_item_async(id).await
        }
    }

    // let c = db.clone();
    // async move |id: i32| {
    //     c.get_item_async(id).await
    // }


    // async move |id: i32| {
    //     db.get_item_async(id).await
    // }

}


#[tokio::main]
async fn main() {
    let db = DBAsync::new("/tmp/cache").await;

    let cl = make_closure_async(db);

    let item = cl(42).await;

    println!("Item: {:?}",item)




}
