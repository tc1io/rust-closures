mod syn;
mod asyn;

pub use syn::DBSync;
pub use asyn::DBAsync;


// Some item to get from database. It is our
// domain entity, it is Clone, Send, Sync
#[derive(Debug)]
pub struct Item(String);

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        Item(String::from(value))
    }
}

