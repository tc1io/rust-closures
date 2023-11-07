// Some item to get from database. It is our
// domain entity, it is Clone, Send, Sync
#[derive(Debug)]
pub struct Item(i32);

// Some dependency that offers a sync. and async.
// retrieval method. It is Clone, Send and Sync
#[derive(Clone)]
pub struct DB;
impl DB {
    pub fn get_item_sync(&self, id:i32) -> Item {
         Item(id)
    }
    pub async fn get_item_async(&self, id:i32) -> Item {
        Item(id)
    }

}