use std::collections::HashMap;

use chrono::NaiveDate;
use tera::{Value, try_get_value};


pub fn is_overdue(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let date = try_get_value!("is_overdue", "date", NaiveDate, value);
    let now = chrono::Local::now().naive_local().date();
    Ok(Value::Bool(now < date))
}
