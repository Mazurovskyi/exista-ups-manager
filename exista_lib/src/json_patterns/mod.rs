
use json::JsonValue;

use std::iter::Iterator;
use std::iter::IntoIterator;
use std::vec::IntoIter;


/// trait provides method to insert data into Json-object
pub trait JsonPattern{
    /// assignes "values" data to empty Json pattern. 
    /// Skips the field if it is already assigned.
    fn assign(&mut self, values: Vec<JsonValue>);
    fn fill(&mut self, values: &mut IntoIter<JsonValue>);
}
impl JsonPattern for JsonValue{

    fn assign(&mut self, values: Vec<JsonValue>) {
        let mut values = values.into_iter();
        self.fill(& mut values)
    }

    fn fill(&mut self, values: &mut IntoIter<JsonValue>){
        
        for (_name, el) in self.entries_mut(){
            
            if el.is_object(){
               el.fill(values);
            }
            else{
                let temp = values.next();
                if el.is_null(){
                    *el = temp.into()    //unwraped data or null
                }
            }
        }
    }
}




