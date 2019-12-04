use crate::advent::advent;

pub struct Day01 {}
impl advent::Solution for Day01
{
    fn part1(&self, input: &str) -> String
    {
        fn fuel(mass: i32) -> i32
        {
            return mass / 3 - 2;
        }

        // for line in io::stdin().lock().lines()
        let result: i32 = input.lines().map(
            |line|
            {
                let input = i32::from_str_radix(line, 10);
                if input.is_err()
                {
                    panic!("Cannot convert '{}'.", line);
                }
                fuel(input.unwrap())
            }).sum();
        result.to_string()
    }

    fn part2(&self, input: &str) -> String
    {
        fn fuel_inner(mass: i32) -> i32
        {
            let result = mass / 3 - 2;
            if result < 0
            {
                0
            }
            else
            {
                result
            }
        }

        fn fuel(mass: i32) -> i32
        {
            let addon = fuel_inner(mass);
            if addon == 0
            {
                0
            }
            else
            {
                addon + fuel(addon)
            }
        }

        let result: i32 = input.lines().map(
            |line|
            {
                let input = i32::from_str_radix(line, 10);
                if input.is_err()
                {
                    panic!("Cannot convert '{}'.", line);
                }
                fuel(input.unwrap())
            }).sum();
        result.to_string()
    }
}
