fn main() {
    // let word = "test";

    // println!("{}", gimme([2, 3, 1]));

    // assert_eq!(0, gimme([2, 3, 1]))
    // println!("Remove vowels: {:?}", remove_vowels(word))
    //  println!("middle is {}", get_middle(word));
}

/*
gimme([2, 3, 1]) => 0

*/

// fn gimme(input_array: [i32; 3]) -> usize {
//     let max = input_array.iter().max();
//     let min = input_array.iter().min();

//     input_array
//         .iter()
//         .position(|i| {
//             if Some(i) > min && Some(i) < max {
//                 true
//             } else {
//                 false
//             }
//         })
//         .unwrap()

//     let mut ret = input_array.clone();
//     ret.sort();
//     input_array.iter().position(|&x| x == ret[1]).unwrap()
// }

// fn parse(code: &str) -> Vec<i32> {
//     code.chars()
//         .fold((0 as i32, Vec::new()), |(current, mut res), x| match x {
//             'i' => (current + 1, res),
//             'd' => (current - 1, res),
//             's' => (current * current, res),
//             'o' => {
//                 res.push(current);
//                 (current, res)
//             }
//             _ => (current, res),
//         })
//         .1
// }

// fn get_middle(word: &str) -> &str {
//     let count = word.len();
//     let from = (count - 1) / 2;
//     let to = (count / 2) + 1;

//     &word[from..to]
// }

// fn remove_vowels(word: &str) -> String {
//     let vowels = vec!["a", "e", "i", "o", "u"];
//     let mut new_word = String::new();

//     word.chars()
//         .map(|letter| {
//             if vowels.iter().any(|c| c.contains(letter)) {
//                 new_word.push(letter)
//             }
//         })
//         .collect();

//     new_word
// }
