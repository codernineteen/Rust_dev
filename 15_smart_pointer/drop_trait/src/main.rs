struct CustomSmartPointer {
    data: String,
}

//Drop trait is included in the prelude
impl Drop for CustomSmartPointer {
    //Drop trait requires you to implement one method named drop that takes a mutable reference to self.
    fn drop(&mut self) {
        println!("Dropping smart pointer with data {}", self.data);
    }
}

fn main() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    //you can't call .drop method explicilty
    //Instead, if you want to drop instance earlier than it dropped automatically,
    //you can use std::mem::drop and this is also included in prelude
    //So you just can call this as 'drop'
    drop(d);
    println!("Instance d was dropped early");
} //Here Rust call drop method automatically when instances goes out of scope
