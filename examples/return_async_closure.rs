#![feature(impl_trait_in_fn_trait_return)]
use std::future::Future;
use rust_closures::*;

fn make_closure_async(db: DB) -> impl Fn(i32) -> impl Future<Output=Item> {
    move |id:i32| {
        let c = db.clone();
        async move {
            c.get_item_async(id).await
        }
    }

}


#[tokio::main]
async fn main() {
    let db = DB;

    let cl = make_closure_async(db);

    let item = cl(42).await;

    println!("Item: {:?}",item)




}
