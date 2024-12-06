use super::utils::read_lines;

pub fn day1() {
    let mut list1  = Vec::new();
    let mut list2 = Vec::new();
    for item in read_lines("inputs/day-1.txt") {
        let parsed_item = String::from(item);
        list1.push(parsed_item[0..5].to_string());

    }
    for item in read_lines("inputs/day-1.txt") {
        let parsed_item = String::from(item);
        list2.push(parsed_item[8..13].to_string());

    }

    list1.sort();
    list2.sort();
    
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
        
        println!("{diff}")
    }


}