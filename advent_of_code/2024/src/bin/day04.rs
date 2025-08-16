use advent2024::{Direction, Tilemap};

fn main() {
    let map: Tilemap<u8> = Tilemap::new_with_row_iterators(std::io::stdin().lines().map(|line| line.unwrap().into_bytes().into_iter()));

    let mut part_1_answer = 0;

    for (coords, tile) in map.coords().zip(map.tiles()) {
        let found_char = *tile;
        
        if found_char != b'X' {
            continue;
        }
        // it was X!
        for dir in Direction::ALL {
            if map.get_tile(coords + dir) == Some(&b'M')
            && map.get_tile(coords + dir * 2) == Some(&b'A')
            && map.get_tile(coords + dir * 3) == Some(&b'S')
            { 
                part_1_answer += 1;
            }
        }
    }

    let mut part_2_answer = 0;
    for (coords, tile) in map.coords().zip(map.tiles()) {
        let found_char = *tile;
        
        if found_char != b'A' {
            continue;
        }
        // it was X!

        if map.get_tile(coords + Direction::NorthWest) == map.get_tile(coords + Direction::SouthEast) {
            continue;
        }
        // if map.get_tile(coords + Direction::SouthWest) == map.get_tile(coords + Direction::NorthEast) {
        //     continue;
        // }
        let mut m_count = 0;
        let mut s_count = 0;
        for dir in Direction::ORDINAL {
            match map.get_tile(coords + dir) {
                Some(&b'M') => m_count += 1,
                Some(&b'S') => s_count += 1,
                _ => (),
            }
        }
        if m_count == 2 && s_count == 2 {
            part_2_answer += 1;
        }
    }
    


    println!("Part 1 solution: {}", part_1_answer);
    println!("Part 2 solution: {}", part_2_answer);
}