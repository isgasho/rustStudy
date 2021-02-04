// pub mod front;

pub mod front_of_house {
    // pub关键字使外部调用可达
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("run add_to_waitlist");
            seat_at_table();
        }

        fn seat_at_table() {
            println!("this is private func seat_at_table")
        }
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }

    pub struct payment {
        value: i32
    }

    impl payment {
        pub fn get(&self) -> i32 { self.value }
    }
}

// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub mod hosting {
    pub fn add_to_waitlist() {}
}