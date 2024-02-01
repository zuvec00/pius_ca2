use crate::std::prelude::*;

//Example 1, using raw Future trait
// DEFINE A CUSTOM FUTURE THAT SIMULATES AN ASYNCHRONOUS OPERATION
pub struct MyFuture {
    value: i32,
}

impl Future for MyFuture {
    type Output = i32;

    // IMPLEMENT THE POLL METHOD TO MANAGE ASYNCHRONOUS EXECUTION
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("PERFORMING ASYNC WORK..."); // SIMULATE ASYNC WORK
        Poll::Ready(self.value) // RETURN THE VALUE AS READY
    }
}

// FUNCTION TO CREATE AND EXECUTE THE FUTURE
pub async fn run_future() {
    let future = MyFuture { value: 75 };
    let result = future.await;
    println!("RESULT: {}", result);
}
//You need an executor to run the above. See main.rs

//Example 2 using async/await from https://os.phil-opp.com/async-await/#implementation
//uses Future under the hood
async fn async_number() -> u32 {
    42
}

pub async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

//Example 3: Let's work with Arc (the Rc equivalent for threadsafe environment) and Mutex for mutual exclusion
//in multitasking environment
use alloc::sync::Arc;
use spin::Mutex;
pub struct SharedData {
    pub value: u32,
}

//Let's wrap the SharedData with Arc for immutable sharing; mutex for internal mutability as required for thread safety
pub struct Wrapper {
    data: Arc<Mutex<SharedData>>, 
}

pub fn modify_data(wrapper: Arc<Wrapper>) {
    let mut lock = wrapper.data.lock();
    lock.value += 10;
    println!("Modified value: {}", lock.value);
}

pub async fn run_modify_data(data: Arc<spin::mutex::Mutex<SharedData>>) {
    let wrapper = Arc::new(Wrapper { data });
    modify_data(wrapper);
}

