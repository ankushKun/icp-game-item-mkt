#[ic_cdk::query]
use std::thread_local;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct sellingItem{
    pub sellerAddress:String,
    pub itemID:u64,
    pub status:bool,
    pub name:String,
    pub price:u64,
    pub copies:u64,
    }

pub struct buyingItem{
    pub buyerAddress:String,
    pub itemId:u64,
    pub copies:u64,
    }

    // let mut numToItem: HashMap<i32, String> = HashMap::new();


thread_local! {
    pub static NUM: RefCell<i32> = RefCell::new(0);

    pub static SELLING_ITEM: RefCell<sellingItem> = RefCell::new(sellingItem {
        sellerAddress: String::new(),
        itemID:0,
        status:false,
        name:String::new(),
        price: 0,
        copies:0,
    });

    pub static BUYING_ITEM: RefCell<buyingItem>=RefCell::new(buyingItem{
        buyerAddress:String::new();
        itemId:0;
        copies:0,
    });

    // pub static sellerAccount: RefCell<HashMap<u64,String>>=RefCell::new(HashMap::new());    
    pub static itemToSeller: RefCell<HashMap<u64,String>>=RefCell::new(HashMap::new());    // item ID to seller Address
    pub static sellItem: RefCell<HashMap<u64,sellingItem>> = RefCell::new(HashMap::new());  // item ID to full item info

}

fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

fn sell(_sellerAddress:String,_itemID:u64,_name:String,_price:u64,_copies:u64) -> (){

    let new_item = SellingItem {
        sellerAddress: _sellerAddress,
        itemID: _itemID,
        status: true, 
        name: _name,
        price: _price,
        copies: _copies,
    };
    
    itemToSeller.with(|map1|{
        let mut map1 = map1.borrow_mut();
        map1.insert(_itemID, _sellerAddress);
    });

     sellItem.with(|map1|{
        let mut map1 = map1.borrow_mut();
        map1.insert(_itemID, new_item);
     });
} 

fn buy(_itemID:u64,_copies:u64,_money:u64) ->(){
       
}
