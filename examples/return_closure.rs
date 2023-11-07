use rust_closures::*;

fn make_closure(db: DB) -> impl Fn(i32) -> Item {
    move |id:i32| {
        db.get_item_sync(id)
    }

}


#[tokio::main]
async fn main() {
    let db = DB;

    let cl = make_closure(db);


    let item = cl(42);

    println!("Item: {:?}",item)




}
