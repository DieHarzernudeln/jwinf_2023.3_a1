use std::io::{BufRead, BufReader};

fn main() {
    let (bag_count, game_count, games) = read_data();

    println!("bags: {}; with {} games. Games: {:?}", bag_count, game_count, games);

    let mut bags = init_bags(bag_count);

    let mut index: usize = 0;
    for game in games {
        let split = game / bag_count;
        for _ in 0..split {
            for bag in 0..bag_count as usize {
                bags[bag].push_str(&game.to_string());
            }
        }

        for _ in 0..(game - (split * bag_count)) {
            let pbags: &mut Vec<String> = &mut bags;
            index = append_asym(index, game, bag_count, pbags);
        }
    }

    println!("{:?}", bags);
}

fn read_data() -> (i32, i32, Vec<i32>) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let file_path = input.trim();
    let file = BufReader::new(std::fs::File::open(file_path).unwrap());

    let mut lines = file.lines();

    let bag_count: i32 = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let game_count: i32 = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    let mut games: Vec<i32> = Vec::new();

    for line in lines {
        games.push(line.unwrap().parse::<i32>().unwrap());
    }

    return (bag_count, game_count, games);
}

fn init_bags(bag_count: i32) -> Vec<String> {
    let mut bags: Vec<String> = Vec::new();
    for _ in 0..bag_count {
        bags.push(String::new());
    }
    return bags;
}

fn append_asym(index: usize, game: i32, bag_count: i32, pbags: &mut Vec<String>) -> usize {
    (*pbags)[index].push_str(&game.to_string());
    if index == (bag_count - 1) as usize {
        0
    } else {
        index + 1
    }
}
