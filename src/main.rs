use clap::{App, Arg, ArgMatches};

fn get_address(matches: ArgMatches) {
    if let Some(i) = matches.value_of("address") {
        println!("You ordered for delivery.");
        println!("Delivery address: {}", i);
    }
}

fn main() {
    let matches = App::new("Burger Builder")
        .version("1.0")
        .author("Jason Lengstorf <jason@lengstorf.com>")
        .about("Helps you build a burger correctly.")
        .arg(
            Arg::new("style")
                .long("style")
                .value_name("BURGER_STYLE")
                .about("What type of burger do you want?")
                .takes_value(true),
        )
        .arg(
            Arg::new("order_type")
                .required(true)
                .long("order-type")
                .value_name("ORDER_TYPE")
                .about("Dine-in, pickup, or delivery?")
                .takes_value(true),
        )
        .arg(
            Arg::new("address")
                .long("address")
                .required_if_eq("order_type", "delivery")
                .value_name("address")
                .about("Where do you live?")
                .takes_value(true),
        )
        .arg(
            Arg::new("toppings")
                .short('t')
                .long("topping")
                .value_name("TOPPING")
                .about("What toppings do you want on your burger?")
                .multiple_occurrences(true)
                .takes_value(true),
        )
        .get_matches();

    if let Some(i) = matches.value_of("style") {
        match i {
            "smash" => println!("You asked for a smash burger."),
            "sous vide" => println!("Nice try, Sarah."),
            _ => println!(
                "You said {}, but we know you really want a smash burger.",
                i
            ),
        }
    }

    if let Some(i) = matches.values_of("toppings") {
        println!("Toppings:");

        let vals: Vec<&str> = i.collect();

        for val in vals {
            match val {
                "lettuce" => println!("- {}", val),
                "pickle" => println!("- {}", val),
                "pineapple" => println!("- {}", val),
                "tomato" => println!("- {}", val),
                "onion" => println!("- {}", val),
                _ => println!("We don’t have {}, sorry.", val),
            }
        }
    }

    if let Some(i) = matches.value_of("order_type") {
        match i {
            "delivery" => get_address(matches),
            _ => println!("You ordered for {}.", i),
        }
    }
}
