mod password;
use password::*;

fn main() {
    // 1150
    println!("Number of potentially valid passwords in given range {}", valid_in_range(240298,784956));

    // 748
    println!("Number of potentially valid passwords (strict) in given range {}", strict_valid_in_range(240298,784956));
}
