//recoverrable - Result<T, E> type
//unrecoverable - panic! macro

//panic! -> by default, unwinding (walks back up the stack and clean up the data from function)
//panic! -> abort(ends the program without cleaning up) -> memory cleaned up by OS
//If you want to change unwinding to aborting add below in Cargo.toml
//[profile.release]
// panic = 'abort'

fn main() {
    //Using a panic! Backtrace
    let v = vec![1, 2, 3];
    v[99]; //This occur panic and it comes from where it defined.
           // To trace of related description about this panic run with `RUST_BACKTRACE=1`

    //The way of avoding panic is to avoid panic case (e,g v[99] when vector only has less indices.)
}

fn _panic_function() {
    //simple panic example
    panic!("crash and burn");
}
