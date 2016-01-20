// https://mobiarch.wordpress.com/2015/07/08/understanding-lifetime-in-rust-part-ii-3/

struct Car {
    model: String
}

struct Person<'a> {
    car:Option<&'a Car>,
}

impl <'a> Person<'a> {

    // Constructor
    fn new() -> Person<'a> {
        Person{
            car: None
        }
    }

    // Buy
    fn buy_car(&mut self, c: &'a Car) {
        self.car = Some(c);
    }

    // Sell
    fn sell_car(&mut self) {
        self.car = None;
    }

    // Trade
    fn trade_with(&mut self, other: &mut Person<'a>) {
        let tmp = other.car;

        other.car= self.car;
        self.car = tmp;
    }

}

fn main() {
    let civic = Car{model: "Honda Civic".to_string()};
    let ghibli = Car{model: "Maserati Ghibli".to_string()};

    let mut bob = Person::new();
    let mut alice = Person::new();

    bob.buy_car(&civic);
    alice.buy_car(&ghibli);

    bob.trade_with(&mut alice);

    match bob.car {
        None => println!("bob doesn't own a car"),
        Some(c) => println!("bob owns the {}",c.model),
    }
}
