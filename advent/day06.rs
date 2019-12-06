use std::collections::HashMap;
use std::collections::HashSet;

fn parse(input: &str) -> HashMap<String, Vec<String>>
{
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines()
    {
        let pair: Vec<&str> = line.split(')').collect();
        let center: String = pair[0].to_string();
        let orbiter: String = pair[1].to_string();

        if map.contains_key(&center)
        {
            let orbiters = map.get_mut(&center).unwrap();
            orbiters.push(orbiter.clone());
        }
        else
        {
            map.insert(center, vec![orbiter.clone()]);
        }
        if !map.contains_key(&orbiter)
        {
            map.insert(orbiter, vec![]);
        }
    }
    map
}

fn reverseMap(map: &HashMap<String, Vec<String>>) -> HashMap<String, String>
{
    let mut result: HashMap<String, String> = HashMap::new();
    for (center, orbiters) in map
    {
        for orbiter in orbiters
        {
            result.insert(orbiter.clone(), center.clone());
        }
    }
    result
}

fn goThrough(map: &HashMap<String, Vec<String>>, root: &String, depth: u32) -> u32
{
    let childs = map.get(root).unwrap();
    let mut orbit_count: u32 = depth;

    for orbiter in childs
    {
        orbit_count += goThrough(map, orbiter, depth + 1);
    }
    orbit_count
}

pub fn part1(input: &str) -> u32
{
    let map = parse(input);
    let center = String::from("COM");

    goThrough(&map, &center, 0)
}

pub fn part2(input: &str) -> usize
{
    let map = parse(input);
    let reverse_map = reverseMap(&map);

    let mut you_path_set: HashSet<String> = HashSet::new();
    let mut you_path: Vec<String> = vec!["YOU".to_string()];
    you_path_set.insert("YOU".to_string());
    let mut san_path_set: HashSet<String> = HashSet::new();
    let mut san_path: Vec<String> = vec!["SAN".to_string()];
    you_path_set.insert("SAN".to_string());


    let mut you: String = String::from("YOU");
    let mut san: String = String::from("SAN");

    loop
    {
        you = reverse_map.get(&you).unwrap().clone();
        you_path.push(you.clone());
        you_path_set.insert(you.clone());

        san = reverse_map.get(&san).unwrap().clone();
        san_path.push(san.clone());
        san_path_set.insert(san.clone());

        if san_path_set.contains(&you)
        {
            return san_path.iter().position(|x| x == &you).unwrap() + you_path.len() - 1 - 2;
        }

        if you_path_set.contains(&san)
        {
            return you_path.iter().position(|x| x == &san).unwrap() + san_path.len() - 1 - 2;
        }
    }
}
