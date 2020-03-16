mod restaurant {
    mod front_of_house {
        struct appetizer {
            toast: String,
            seasonal_fruit: String,
        }
        enum dessert {
            ice_cream,
            coffee,
        }

        mod hosting {
            fn add_to_waitlist() {}
            fn seat_at_table() {}
        }
        mod serving {
            fn take_order() {}
            fn serve_order() {}
            fn take_payment() {}
        }
    }
    mod back_of_house {
        mod manage_restaurant {
            fn count_payment() {}
            fn clean_hall() {}
        }
        mod order_ingredient {
            fn check_refrigerator() {}
            fn check_warehouse() {}
            fn make_order_list() {}
        }
    }
}

fn eat_at_restaurant() {

}

fn main() {

}