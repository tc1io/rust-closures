use rust_closures::*;

fn make_closure(db: DBSync) -> impl Fn(i32) -> Item {
    //let mut db2 = db.clone();
    move |id:i32| {
        let mut db3 = db.clone();
        db3.put_item_sync(id,Item::from("my sync item"));
        db3.get_item_sync(id)
    }

}


#[tokio::main]
async fn main() {
    let db = DBSync::new("/tmp/test");

    let cl = make_closure(db);


    let item = cl(42);

    println!("Item: {:?}",item)




}
