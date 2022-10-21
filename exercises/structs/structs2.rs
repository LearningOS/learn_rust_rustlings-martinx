// structs2.rs
// Address all the TODOs to make the tests pass!

#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

impl Order {
    fn create_order_template() -> Order {
        Order {
            name: String::from("Bob"),
            year: 2019,
            made_by_phone: false,
            made_by_mobile: false,
            made_by_email: true,
            item_number: 123,
            count: 0,
        }
    }  
    
    fn duplicate(&self, name: &str, count: u32) -> Order{
        Order {
            name: name.to_string(),
            year: 2019,
            made_by_phone: self.made_by_phone,
            made_by_mobile: self.made_by_mobile,
            made_by_email: self.made_by_email,
            item_number: self.item_number,
            count: count,
        }
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn your_order() {
        let order_template = Order::create_order_template();
        let your_order = order_template.duplicate("Hacker in Rust", 1);

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}