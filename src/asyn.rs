use std::rc::Rc;
use forceps::Cache;
use super::Item;


// Some dependency that offers a sync. and async.
// retrieval method. It is Clone, Send and Sync
#[derive(Clone)]
pub struct DBAsync {
    cache: Rc<Cache>

}
impl DBAsync {
    pub async fn new(filename: &str) -> DBAsync {
        let cache = Cache::new(filename)
            .build()
            .await.unwrap();

        DBAsync {cache: Rc::new(cache)}
    }
    pub async fn put_item_async(&mut self, id:i32, item: Item) {
        self.cache.write(id.to_string().as_bytes(), item.0.as_bytes()).await.unwrap();
            ()
    }
    pub async fn get_item_async(&self, id:i32) -> Item {
        let data = self.cache.read(id.to_string().as_bytes()).await.unwrap();
         Item(String::from_utf8(data.to_vec()).unwrap())
    }

}
