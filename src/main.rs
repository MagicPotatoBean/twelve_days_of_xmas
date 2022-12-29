const LYRICS: [&str; 11] = [
    "a partridge in a pear tree",
    "Two turtledoves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings (five golden rings)",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
fn main() {
    for day in 0..=11 {
        if day == 0 {
            println!("On the 1st day of christmas my true love gave to me ");
        } else if day == 1 {
            println!("On the 2nd day of christmas my true love gave to me ");
        } else if day == 2 {
            println!("On the 3rd day of christmas my true love gave to me ");
        } else {
            println!(
                "On the {}th day of christmas my true love gave to me ",
                day + 1
            );
        }

        for item in (0..=day).rev() {
            if (item == 0) && (day != 0) {
                println!("and {}", LYRICS[item]);
            } else {
                println!("{}", LYRICS[item]);
            }
        }
        println!("");
    }
}