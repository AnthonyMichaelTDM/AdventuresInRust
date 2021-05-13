fn main() {
    /*
    unwinding the stack, or aborting in response to a panic,

    By default, when a panic occurs, the program starts unwinding, 
    which means Rust walks back up the stack and cleans up the data from each function it encounters. 
    But this walking back and cleanup is a lot of work. The alternative is to immediately abort, 
    which ends the program without cleaning up.

    If in your project you need to make the resulting binary as small as possible, 
    you can switch from unwinding to aborting upon a panic by adding panic = 'abort' 
    to the appropriate [profile] sections in your Cargo.toml file. 

    For example, if you want to abort on panic in release mode, add this:

    [profile.release]
    panic = 'abort'
    */

    //this will immediatly crash the program
    //panic!("crash and burn");

    //another way to cause a crash is by referencing an invalid index

    //let v = [1,2,3];
    //println!("{}", v[99]);
}
