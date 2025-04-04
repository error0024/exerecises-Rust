// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

impl Order {
    pub fn new(new_name: String, new_quantity: u32, new_price: u32) -> Order {
        if new_name.is_empty() {
            panic!("The name can not be empty!");
        }
        if new_name.len() > 300 {
            panic!("The name can not be longer than 300 bytes!");
        }
        if new_quantity == 0 {
            panic!("The quantity must be greater than zero!");
        }
        if new_price == 0 {
            panic!("The price must be greater than zero!");
        }
        Order{
            product_name: new_name,
            quantity: new_quantity,
            unit_price: new_price
        }

    }
    
    pub fn total(&self) -> u32 {
        let total = self.quantity * self.unit_price;
        total
    }

    pub fn set_product_name(&mut self, new_name: String)  {
        if new_name.is_empty() {
            panic!("Name cannot be empty");
        }
        if new_name.len() > 300 {
            panic!("Title cannot be longer than 300 bytes");
        }
        self.product_name = new_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u32)  {
        if new_quantity == 0 {
            panic!("The quantity must be greater than zero!");
        }
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_price: u32)  {
        if new_price == 0 {
            panic!("The price must be greater than zero!");
        }
        self.unit_price = new_price;
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }
}