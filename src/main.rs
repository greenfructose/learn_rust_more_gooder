// Bookmark https://doc.rust-lang.org/book/ch08-03-hash-maps.html
use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 3, 8, 65, 4, 5, 6, 7];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    for i in &v {
        println!("{} is in v", i);
    }
    for i in &mut v {
        *i += 50;
    }
        for i in &v {
        println!("{} is in v", i);
    }
    println!("Mean, median, mode of v2 = {:?}", mean_median_mode(v2));
    println!("{}", igpay_atinlay(&String::from("My name is Justin")))
}

fn find_mean(list: &Vec<i32>) -> i32 {
    let mut total = 0;
    for i in list {
      total  += i;
    };
    total / list.len() as i32
}

fn mean_median_mode(mut list: Vec<i32>) -> HashMap<String, i32> {
    list.sort();
    let mut map = HashMap::new();
    let mut total = 0;
    let mut mode_map = HashMap::new();
    for i in &list {
        total += i;
        let count = mode_map.entry(i).or_insert(0);
        *count += 1;
    };
    let mean = total / list.len() as i32;
    map.insert(String::from("Mean"), mean);
    let median_index = (list.len()as i32 / 2) as usize;
    let median = list[median_index];
    map.insert(String::from("Median"), median);
    let mode = {
        let mut x = 0;
        let mut y = 0;
        for(key, value) in &mode_map {
            if value > &x {
                x = **&value;
                y = **key;
            }
        };
        y
    };
    map.insert(String::from("Mode"), mode);
    map
}

fn igpay_atinlay(string: &String) -> String {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut words = string.split_whitespace();
    let mut new_word = String::new();
    let mut new_words = Vec::new();
    for word in words{
        let char = word.chars().next().unwrap();
        if vowels.contains(&char){
            new_word.push_str(&*format!("{}hay", word));
        }else{
            new_word = String::from(&word[1..]);
            let mut suffix = word.chars().next().unwrap().to_string().to_lowercase() + &String::from("ay");
            if char.is_uppercase() {
                let mut first_char = &new_word.chars().next().unwrap().to_string().to_uppercase();
                new_word = format!("{}{}", first_char.to_string(), &word[2..]);
            }
            new_word.push_str(&suffix);
        }
        new_words.push(String::from(&new_word));
        new_word.clear();
    }
    new_words.join(" ")

}