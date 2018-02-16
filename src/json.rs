extern crate serde_json;

use serde_json::Value;
use operations::Operation;

struct JSON {
    value:Value,
}

trait ApplyOperation{

    fn apply(operation:Operation)->();

}

impl ApplyOperation for JSON {

    fn apply(operation:Operation)->(){
        let mutation = operation
        match operation {
            Operation::
        }
    }

}