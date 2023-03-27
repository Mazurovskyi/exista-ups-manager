use std::sync::Mutex;
use std::collections::VecDeque;
use std::borrow::Borrow;

use once_cell::sync::Lazy;
//use std_semaphore::Semaphore;
use os_sync::Sem;
use os_sync::Semaphore;
use super::Request;



/// stores Request obects that have to be sended over Mqtt.
pub struct RequestsStack(Stack);
impl RequestsStack{

    /// push the new request into requests stack and increments the internal count of the semaphore.
    pub fn push(request: Request)->Result<(), String>{
        unsafe{
            STACK.data().lock().or_else(|err|Err(err.to_string()))?.push_back(request);
            STACK.status().signal()
        }
        Ok(())
    }

    /// blocks the current thread until the internal count of the semaphore is at least 1.
    pub fn pull()->Result<Request, String>{
        unsafe{

            STACK.status().wait();

            STACK.data().lock()
                .or_else(|err|Err(err.to_string()))?
                .pop_front()
                .ok_or("requests stack is empty but semaphore was released.".into())
        }
    }
}




static mut STACK: Lazy<Stack> = Lazy::new(||Stack::default());

struct Stack{
    status: Sem,
    data: Mutex<VecDeque<Request>>
}
impl Stack{
    fn status(&self)->&Sem{
        self.status.borrow()
    }
    fn data(&self)->&Mutex<VecDeque<Request>>{
        self.data.borrow()
    }
}
impl Default for Stack{
    fn default() -> Self {
        Self {
            status: Sem::new(0).unwrap(), 
            data: Mutex::new(VecDeque::new())
        }
    }
}







