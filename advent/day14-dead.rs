use std::vec::Vec;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use crate::lib::ratio;

type CountType = u64;
type TreeType = HashMap<String, (Vec<ReactionInput>, CountType)>;
type QuantityMap = HashMap<String, ratio::Ratio<CountType>>;

#[derive(Debug)]
struct ReactionInput
{
    name: String,
    quantity: ratio::Ratio<CountType>,
}

fn parse(input: &str, write_dot: &mut Option<File>) -> TreeType
{
    let mut result = TreeType::new();

    if let Some(file) = write_dot
    {
        writeln!(file, "digraph G {{");
    }

    for line in input.lines()
    {
        let parts: Vec<&str> = line.split("=>").collect();
        let parts_right: Vec<&str> = parts[1].trim().split(' ').collect();
        let product_count: CountType = parts_right[0].trim().parse().unwrap();
        let product: &str = parts_right[1].trim();

        let mut inputs = Vec::new();
        for input_str in parts[0].split(',')
        {
            let parts_input: Vec<&str> = input_str.trim().split(' ').collect();
            let input_count: CountType = parts_input[0].trim().parse().unwrap();
            let input_name: &str = parts_input[1].trim();
            inputs.push(
                ReactionInput
                {
                    name: String::from(input_name),
                    quantity: ratio::Ratio::from(input_count, product_count).unwrap(),
                });

            if let Some(file) = write_dot
            {
                writeln!(file, "{} -> {};", input_name, product);
            }
        }

        result.insert(String::from(product), (inputs, product_count));
    }

    if let Some(file) = write_dot
    {
        writeln!(file, "}}");
    }
    result
}

fn findQuantityInner<'a>(product: &str, quantity: ratio::Ratio<CountType>, reactions: &'a TreeType)
                         -> Vec<(&'a str, ratio::Ratio<CountType>)>
{
    if product == "ORE"
    {
        return vec![];
    }

    let mut result: Vec<(&'a str, ratio::Ratio<CountType>)> = Vec::new();
    for input in &reactions.get(product).unwrap().0
    {
        let num = quantity * input.quantity;
        result.push((&input.name, num));
        let mut later = findQuantityInner(&input.name, num, &reactions);
        result.append(&mut later);
    }
    result
}

fn findQuantity(product: &str, quantity: ratio::Ratio<CountType>, reactions: &TreeType)
                    -> QuantityMap
{
    let mut result: QuantityMap = HashMap::new();

    for item in &findQuantityInner(product, quantity, reactions)
    {
        result.insert(
            String::from(item.0),
            result.get(item.0).map_or(ratio::Ratio::zero(), |x| *x)
                + item.1);
    }
    result
}

fn addQuantityTo(base: &mut QuantityMap, delta: &QuantityMap)
{
    for (name, quantity) in delta
    {
        base.insert(name.clone(),
                    base.get(name).map_or(quantity.clone(), |x| *x + quantity.clone()));
    }
}

fn refineQuantityInner(raw_quantities: &QuantityMap, reactions: &TreeType)
                       -> QuantityMap
{
    let mut adjust: QuantityMap = QuantityMap::new();
    for (name, quantity) in raw_quantities
    {
        if name == "ORE"
        {
            continue;
        }

        let increment = reactions.get(name).unwrap().1;
        if quantity.isInteger() && (quantity.numeritor % increment == 0)
        {
            continue;
        }
        let ceil = quantity.ceil();
        let adjusted = if ceil % increment == 0
        {
            ceil
        }
        else
        {
            (ceil / increment + 1) * increment
        };

        adjust.insert(
            name.clone(),
            adjust.get(name).map_or(ratio::Ratio::fromInt(adjusted),
                                    |x| *x + ratio::Ratio::fromInt(adjusted)));
    }

    let mut delta: QuantityMap = QuantityMap::new();
    for (name, quantity) in &adjust
    {
        addQuantityTo(&mut delta, &findQuantity(name, quantity.clone(), reactions));
    }
    addQuantityTo(&mut adjust, &delta);
    adjust
}

fn refineQuantity(mut raw_quantities: QuantityMap, reactions: &TreeType)
                  -> QuantityMap
{
    loop
    {
        let adjusted = refineQuantityInner(&raw_quantities, reactions);
        if adjusted.is_empty()
        {
            break;
        }

        for (name, quantity) in adjusted
        {
            raw_quantities.insert(name, quantity);
        }
    }
    raw_quantities
}

fn findOreQuantity(quantities: &QuantityMap, reactions: &TreeType) -> CountType
{
    let mut total: ratio::Ratio<CountType> = ratio::Ratio::zero();

    for (name, quantity) in quantities
    {
        if name == "ORE"
        {
            continue;
        }

        for input in &reactions.get(name).unwrap().0
        {
            if input.name == "ORE"
            {
                total = total + findQuantity(name, quantity.clone(), reactions).get("ORE").unwrap().clone();
            }
        }
    }
    total.numeritor
}

pub fn part1(input: &str) -> CountType
{
    let tree = parse(input, &mut None);
    let quantities_raw = findQuantity("FUEL", ratio::Ratio::fromInt(1), &tree);
    let quantities = refineQuantity(quantities_raw, &tree);
    findOreQuantity(&quantities, &tree)
}

pub fn part2(input: &str) -> CountType
{
    0
}

#[test]
fn testPart1()
{
    assert_eq!(part1("1 ORE => 2 A
1 A => 1 B
1 A, 1 B => 1 FUEL"), 1);

    assert_eq!(part1("1 ORE => 1 A
1 A => 2 B
3 B => 1 C
1 B, 1 C => 1 FUEL"), 2);

    assert_eq!(part1("10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"), 31);

    assert_eq!(part1("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"), 165);

    assert_eq!(part1("2 ORE => 2 C
2 ORE => 3 D
1 ORE => 1 E
5 ORE => 5 F
1 D, 1 C => 1 A
2 D, 3 C => 1 B
2 E, 6 F, 3 A, 1 B => 1 FUEL"), 22);

    {
        let mut dot_file = File::create("test.dot").unwrap();
        let tree = parse("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT", &mut Some(dot_file));
        let quantities_raw = findQuantity("FUEL", ratio::Ratio::fromInt(1), &tree);
        let quantities = refineQuantity(quantities_raw, &tree);
        assert_eq!(findOreQuantity(&quantities, &tree), 13312);
    }

    assert_eq!(part1("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"), 13312);

    assert_eq!(part1("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"), 180697);

    assert_eq!(part1("171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"), 2210736);
}
