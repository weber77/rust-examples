mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::hosting::seat_at_table();

    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn main() {
    println!("Hello, world!");
}
