use std::cell::RefCell;

thread_local! {
    static MSG: RefCell<Vec<String>> = RefCell::new(Vec::new());
} 

#[ic_cdk::update]
fn set_msg(new_msg: String) {
    CHAT.with(|chat| {
        chat.borrow_mut().push(new_msg) 
    })
}

#[ic_cdk::query]
fn get_msg() -> String {
    CHAT.with(|chat| chat.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}



 // new_msg: String
    // msg.borrow_mut(): &mut:String
    // *&mut:String != string
    // string == string