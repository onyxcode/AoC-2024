use super::utils;

pub fn day1() {
    let mut list1  = Vec::new();
    let mut list2 = Vec::new();
    
    for item in utils::read_lines("inputs/day-1.txt") {
        let parsed_item = String::from(item);
        list1.push(parsed_item[0..5].to_string());
        
    }
    for item in utils::read_lines("inputs/day-1.txt") {
        let parsed_item = String::from(item);
        list2.push(parsed_item[8..13].to_string());
        
    }
    
    list1.sort();
    list2.sort();
    
    // Part 1
    let mut x = 0;
    let list_length = list1.len() - 1;
    let mut diff = 0;

    while x <= list_length {
        diff += (list1[x]
            .parse::<i32>()
            .unwrap() - list2[x]
            .parse::<i32>()
            .unwrap())
            .abs();

        x += 1;
        
    }

    println!("Part 1 (Distance between L and R):\n{diff}");

    // Part 2
    x = 0;
    let mut total_matches = 0;
    while x <= list_length {
        let source = list1[x]
        .parse::<i32>()
        .unwrap();
        let mut matches = 0;
        
        for item in &list2 {
            let item = item.parse::<i32>().unwrap();
            if item == source {
                matches += 1;
            }
        }
        if !(matches == 0) {
            matches = source*matches;
            total_matches += matches;
        }
        x += 1;
    }
    println!("Part 2 (Similarity Score):\n{}", total_matches);


}

pub fn day2() {
    // Part 1
    let mut safe_arrays = 0;
    let mut is_safe: bool;
    for item in utils::read_lines("inputs/day-2.txt") {
        let parts = item.split(" ");
        let array_unparsed: Vec<&str> = parts.collect();
        let mut array: Vec<i32> = Vec::new();
        for item in array_unparsed {
            array.push(item.parse::<i32>().unwrap());
        }
        let array_length = array.len() - 1;
        let mut array_clone = array.clone();
        if array.is_sorted() && !utils::has_duplicates(&array) {
                is_safe = true;
                for x in 1..=array_length {
                    if !is_safe { break }
                    if !(x == 0) {
                        let current = array[x];
                        let previous = array[x-1];
                        if (current - previous).abs() > 3 {
                            println!("From array {:?}\n{} -> {} | unsafe!", array, previous, current);
                            is_safe = false;
                        }
                    }
                }
                if is_safe { safe_arrays += 1}
        } else {
            array_clone.reverse();
            if array_clone.is_sorted() && !utils::has_duplicates(&array) {
                    is_safe = true;
                    for x in 1..=array_length {
                        if !is_safe { break }
                        if !(x == 0) {
                            let current = array[x];
                            let previous = array[x-1];
                            if (current - previous).abs() > 3 {
                                println!("From array {:?}\n{} -> {} | unsafe!", array, previous, current);
                                is_safe = false;
                            }
                        }
                    }
                    if is_safe { safe_arrays += 1}
            }
        }
        }
    println!("{} safe arrays found.", safe_arrays)
    }