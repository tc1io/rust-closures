use std::rc::Rc;
use std::sync::{Arc, Mutex};
use forceps::Cache;
use super::Item;


// Some dependency that offers a sync. and async.
// retrieval method. It is Clone, Send and Sync
#[derive(Clone)]
pub struct DBAsync {
    cache: Arc<Mutex<Cache>>

}
impl DBAsync {
    pub async fn new(filename: &str) -> DBAsync {
        let cache = Cache::new(filename)
            .build()
            .await.unwrap();

        DBAsync {cache: Arc::new(Mutex::new(cache))}
    }
    pub async fn put_item_async(&mut self, id:i32, item: Item) {
        self.cache.lock().unwrap().write(id.to_string().as_bytes(), item.0.as_bytes()).await.unwrap();
            ()
    }
    pub async fn get_item_async(&self, id:i32) -> Item {
        let data = self.cache.lock().unwrap().read(id.to_string().as_bytes()).await.unwrap();
         Item(String::from_utf8(data.to_vec()).unwrap())
    }

}
