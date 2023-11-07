use std::ops::Deref;
use std::rc::Rc;
use mhdb::{Db, prelude::*};
use super::Item;

// Some dependency that offers a sync. and async.
// retrieval method. It is Clone, Send and Sync
#[derive(Clone)]
pub struct DBSync {
    db: Rc<Db>
}



impl DBSync {
    pub fn new(filename: &str) -> DBSync {
        let db = Db::open(filename).unwrap();
        DBSync {db: Rc::new(db)}
    }
    pub fn put_item_sync(&mut self, id:i32, item: Item) {
        Rc::get_mut(&mut self.db).unwrap().store(id.to_string(), item.0).unwrap();
        ()
    }
    pub fn get_item_sync(&mut self, id:i32) -> Item {
        let data:Vec<u8> = Rc::get_mut(&mut self.db).unwrap().fetch(id.to_string()).unwrap().unwrap();
        Item(String::from_utf8(data.to_vec()).unwrap())
    }

}