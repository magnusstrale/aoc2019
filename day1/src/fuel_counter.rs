
pub fn calculate_fuel(mass: i64) -> i64 {
    (mass / 3) - 2
}

pub fn calculate_adjusted_fuel(mass: i64) -> i64 {
    let fuel = calculate_fuel(mass);
    if fuel <= 0 { 0 } else { fuel + calculate_adjusted_fuel(fuel) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_mass_12_calculate_fuel_should_give_2() {
        let fuel = calculate_fuel(12);

        assert_eq!(fuel, 2);
    }

    #[test]
    fn given_mass_14_calculate_fuel_should_give_2() {
        let fuel = calculate_fuel(14);

        assert_eq!(fuel, 2);
    }

    #[test]
    fn given_mass_1969_calculate_fuel_should_give_654() {
        let fuel = calculate_fuel(1969);

        assert_eq!(fuel, 654);
    }

    #[test]
    fn given_mass_100756_calculate_fuel_should_give_33583() {
        let fuel = calculate_fuel(100756);

        assert_eq!(fuel, 33583);
    }

    #[test]
    fn given_mass_14_calculate_adjusted_fuel_should_give_2() {
        let fuel = calculate_adjusted_fuel(14);

        assert_eq!(fuel, 2);
    }

    #[test]
    fn given_mass_1969_calculate_adjusted_fuel_should_give_966() {
        let fuel = calculate_adjusted_fuel(1969);

        assert_eq!(fuel, 966);
    }

    #[test]
    fn given_mass_100756_calculate_adjusted_fuel_should_give_50346() {
        let fuel = calculate_adjusted_fuel(100756);

        assert_eq!(fuel, 50346);
    }

}
