use std::sync::Mutex;
use std::collections::VecDeque;
use std::error::Error;
use std::borrow::Borrow;

use once_cell::sync::Lazy;
use std_semaphore::Semaphore;

use super::Request;



pub struct RequestsStack(Stack);
impl RequestsStack{

    /// push the new request into requests stack and increments the internal count of the semaphore.
    pub fn push(request: Request)->Result<(), Box<dyn Error>>{
        unsafe{
            STACK.data().lock().unwrap().push_back(request);
            STACK.status().lock().unwrap().release()
        }
        Ok(())
    }

    /// blocks the current thread until the internal count of the semaphore is at least 1.
    pub fn pull()->Result<Request, Box<dyn Error>>{
        unsafe{
            STACK.status().lock().unwrap().access();
            STACK.data().lock().unwrap().pop_front()
                .ok_or("requests stack is empty but semaphore was released.".into())
        }
    }
}



//static mut STACK: Lazy<Mutex<Vec<Request>>> = Lazy::new(||Mutex::new(Vec::new()));
static mut STACK: Lazy<Stack> = Lazy::new(||Stack::default());

struct Stack{
    status: Mutex<Semaphore>,
    data: Mutex<VecDeque<Request>>
}
impl Stack{
    fn status(&self)->&Mutex<Semaphore>{
        self.status.borrow()
    }
    fn data(&self)->&Mutex<VecDeque<Request>>{
        self.data.borrow()
    }
}
impl Default for Stack{
    fn default() -> Self {
        Self { 
            status: Mutex::new(Semaphore::new(0)), 
            data: Mutex::new(VecDeque::new())
        }
    }
}