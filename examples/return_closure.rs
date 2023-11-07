use rust_closures::*;

fn make_closure(db: DBSync) -> impl Fn(i32) -> Item {
    let mut db = db.clone();
    move |id:i32| {
        db.get_item_sync(id)
    }

}


#[tokio::main]
async fn main() {
    let db = DBSync::new("/tmp/test");

    let cl = make_closure(db);


    let item = cl(42);

    println!("Item: {:?}",item)




}
