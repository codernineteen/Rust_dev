//If rust compiler doesn't have enough information, it will reject code.
use std::slice;
//although the code is okay.
//we can use unsafe code to tell compiler 'this is okay'
//The downside of unsafe code is if unsafe code is incorrect, it will cause problems.

//What unsafe rust code can do is
//1. dereference a raw pointer
//2. call an unsafe function or method
//3. access or modify a mutable static variable
//4. implement an unsafe trait
//5. access fields of union S

//Important : unsafe doesn't turn off borrow checker or disable any other rust's safety
//your role is to manage unsafe code under a valid way for memory safety
//By the way, By managing unsafe code in 'unsafe' block, you can detect problems easily

fn main() {
    //#1
    //dereference a raw pointer
    //raw pointer can be both immutable(*const T) or mutable(*mut T)
    //Different from references and smart pointers :
    //1. allowed to ignore borrowing rules: simultaneous immut and mut pointer / multiple mut pointers
    //2. allowed to be null
    //3. not guaranteed to point to valid memory
    //4. don't implement any automatic cleanup

    //you don't have to use unsafe keyword when you declare references with raw pointer
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    //no guarantee case
    //let address = 0x012345usize; -> arbitrary pointer address
    //let r = address as *const i32 (if pointer points to wrong address, there is no memory access)
    
    //we can't dereference outside of unsafe block
    //To access actual value, we dereference
    unsafe {
        println!("{}", *r1);
        println!("{}", *r2);
    }

    //#2
    //calling an unsafe function or method
    unsafe fn dangerous() {

    }
    
    unsafe {
        //By inserting function call in unsafe block, we tell compiler that we know how to use function properly
        dangerous();
    }

    //#3
    //creating a safe abstraction over unsafe code
    //Remember that slice is a pointer to some data
    //safe abstraction
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
        let len = slice.len(); //slice length
        let ptr = slice.as_mut_ptr(); //method to access the raw pointer of slice

        assert!(mid <= len); //prevent the case that mid may be bigger than array length
        //unsafe code
        unsafe {
            (
                //this method takes raw pointer of slice and slice length
                slice::from_raw_parts_mut(ptr, mid),
                //add method to get a raw pointer that starts at mid
                slice::from_raw_parts_mut(ptr.add(mid), len-mid)
            )
        }
    }

    let mut array_origin = vec![1,2,3,4,5];
    let slice_array = &mut array_origin;
    //we can call this function because unsafe code is defined in safe abstraction
    let (one, two) = split_at_mut(slice_array, 3);
    println!("{:?}", one);
    println!("{:?}", two);

    //#4
    //function declared within extern blocks are always unsafe to call from Rust code
    unsafe {
        println!("absolute value of -3 is {}", abs(-3));
    }

    //To use this code in another programming language , we must disable the runst compiler's name mangling
    //Name mangling is when a compiler changes the name we give to function to a different name that contains more information for compiling
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    //#5
    println!("name is: {}", HELLO_WORLD);

    //change static variable value
    add_to_count(3);
    unsafe {
        //any code that reads or writes from static must be within unsafe block
        println!("{}", COUNTER);
    }
    
}

//#4
//To interact with code written in another language
//you can use 'extern' keyword - Foriegn Function Interface(FFI)
//This abs function is from C standard library
//"C" is application binary interface
//"C" ABI defines how to call the function at the assembly level
extern "C" {
    fn abs(input: i32) -> i32;
}

//#5
//accessing or modifying a mutable static variable
//In rust, global variables are called static variables
//static variable is similar to constant variable (with uppercase naming)
//It can only store references with 'static lifetime
//Using static variable always access the same data(a fixed address in memory)
static HELLO_WORLD: &str = "Hello world";

//Another difference between const and static is that static can be mutable
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

//#6
//unsafe trait
unsafe trait Foo {
    //method definition
}
//By using  unsafe impl, we promise that we'll uphold the invariants that the compiler can't verify
//If we implemnt a type such as raw pointers(is not a Send or Sync)
//and we want to mark that type as Send or Sync
//we must use unsafe block
unsafe impl Foo for i32 {
    //method implementations
}
