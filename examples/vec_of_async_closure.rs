#![feature(impl_trait_in_fn_trait_return)]
//#![feature(async_closure)]
use std::future::Future;
use rust_closures::*;

pub trait Callback {
    type Output;

    fn call(&self, args: i32) -> Self::Output;
}

impl<F, R> Callback for F
    where
        F: Fn(i32) -> R,
{
    type Output = R;

    #[inline]
    fn call(&self, args: i32) -> Self::Output {
        (*self)(args)
    }
}



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

    let v:Vec<Box<dyn Callback<Output=Future<Output=Item>>>> = Vec::new();

    let cl0 = make_closure_async(db);
    let cl1 = make_closure_async(db);
    let cl2 = make_closure_async(db);

    v.push(Box::new(cl0));
    v.push(Box::new(cl1));
    v.push(Box::new(cl2));

    let cl = v.pop().unwrap();

    let item = cl(42).await;

    println!("Item: {:?}",item)




}
