use std::collections::HashMap;
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut count_success = 0;
//     let mut count_played = 0;
//     let mut answer_used = Vec::new();
//     let mut count_success_loop = 0;
//     let mut used_word_frequency = HashMap::new();

//     if !is_tty {
//         let _ = tty();
//     } else {
//         // let mut success = 0;
//         let mut w_mode = false;
//         let mut answer = String::new();
//         let matches = command!() // requires `cargo` feature
//         .version("0.1.0")
//         .about("a simple wordle game")
//         .arg(Arg::new("word")
//             .short('w')
//             .long("word")
//             .value_name("WORD")
//             .help("Sets the word to guess")
//             .required(false)
//             .value_parser(value_parser!(String)))
//         .arg(Arg::new("random")
//             .short('r')
//             .long("random")
//             .help("random mode")
//             .required(false)
//             .action(ArgAction::SetTrue))
//         .arg(Arg::new("difficult")
//             .short('D')
//             .long("difficult")
//             .help("start difficult mode")
//             .required(false)
//             .action(ArgAction::SetTrue))
//         .arg(Arg::new("stats")
//             .short('t')
//             .long("stats")
//             .help("print the state of the game")
//             .action(ArgAction::SetTrue)
//             .required(false))
//         .arg(Arg::new("day")
//             .short('d')
//             .long("day")
//             .value_name("DAY")
//             .help("how many rounds you want to loop")
//             .required(false)
//             .value_parser(value_parser!(usize)))
//         .arg(Arg::new("seed")
//             .short('s')
//             .long("seed")
//             .value_name("SEED")
//             .help("seed for random")
//             .required(false)
//             .value_parser(value_parser!(i32)))
//         .arg(Arg::new("final-set")
//             .short('f')
//             .long("final-set")
//             .value_name("FINAL_SET")
//             .help("final set of words")
//             .required(false)
//             .value_parser(value_parser!(String)))
//         .arg(Arg::new("acceptable-set")
//             .short('a')
//             .long("acceptable-set")
//             .value_name("ACCEPTABLE_SET")
//             .help("acceptable set of words")
//             .required(false)
//             .value_parser(value_parser!(String)))
//         .arg(Arg::new("state")
//             .short('S')
//             .long("state")
//             .value_name("STATE")
//             .help("make the result into a json")
//             .required(false)
//             .value_parser(value_parser!(String)))
//         .arg(Arg::new("config")
//             .short('c')
//             .long("config")
//             .value_name("CONFIG")
//             .help("config file")
//             .required(false)
//             .value_parser(value_parser!(String)))
//         .get_matches();
    
//         let mut default_config = Config{
//             random : Some(false),
//             difficult : Some(false),
//             stats : Some(false),
//             day : Some(1),
//             seed : Some(42),
//             final_set : Some("".to_string()),
//             acceptable_set : Some("".to_string()),
//             state : Some("".to_string()),
//             word : Some("".to_string()),
//         };
//         if let Some(config) = matches.get_one::<String>("config"){
//             default_config = json_to_config(config.to_string()).unwrap();
//         }
//         if let Some(seed) = matches.get_one::<i32>("seed"){
//             default_config.seed = Some(*seed);
//         }
//         if let Some(day) = matches.get_one::<usize>("day"){
//             default_config.day = Some(*day);
//         }
//         if matches.get_flag("difficult"){
//             default_config.difficult = Some(true);
//         }
//         if matches.get_flag("random"){
//             default_config.random = Some(true);
//         }
//         if matches.get_flag("stats"){
//             default_config.stats = Some(true);
//         }
//         if let Some(final_set)= matches.get_one::<String>("final-set"){
//             default_config.final_set = Some(final_set.clone());
//         }
//         if let Some(acceptable_set) = matches.get_one::<String>("acceptable-set"){
//             default_config.acceptable_set = Some(acceptable_set.clone());
//         }
//         if let Some(state) = matches.get_one::<String>("state"){
//             default_config.state = Some(state.clone());
//         }
//         if let Some(word) = matches.get_one::<String>("word"){
//             default_config.word = Some(word.clone());
//         }
        
//         let mut final_set = Vec::new();
//         let mut acceptable_set = Vec::new();
//         let mut temp1 = String::new();
//         let mut temp2 = String::new();

//         if matches.contains_id("final-set"){
//             final_set = read_lines_from_file(default_config.final_set.clone().unwrap(), &mut temp1).unwrap();
//         }
//         else{
//             final_set = select::FINAL.to_vec();
//         }

//         if matches.contains_id("acceptable-set"){
//             acceptable_set = read_lines_from_file(&default_config.acceptable_set.unwrap(), &mut temp2).unwrap();
//         }
//         else{
//             acceptable_set = select::ACCEPTABLE.to_vec();
//         }
        
//         let mut json = State{
//             total_rounds : Some(0),
//             games : Some(Vec::new()),
//         };
//         if matches.contains_id("state"){
//             let data = fs::read_to_string(&default_config.state.unwrap()).unwrap();
//             json = serde_json::from_str(&data).unwrap();
//         }

//         if !is_subset(&final_set, &acceptable_set){
//             panic!("INVALID");
//         }

//         let mut _flag = true;

//         if matches.get_flag("random") && matches.contains_id("word"){
//             panic!("INVALID")
//         }
//         if matches.contains_id("word") && (matches.contains_id("seed") || matches.contains_id("day")){
//             panic!("INVALID")
//         }

//         if matches.contains_id("word"){
//             match matches.get_one::<String>("word") {                   //Judge the mode 1.write with word 2.write without word(input word in terminal) 3.random mode
//                 Some(write_value) => {
//                     answer = write_value.clone();
//                     if !acceptable_set.contains(&answer.as_str()){
//                         panic!("INVALID");
//                     }
//                     let  (success, _gusses ,frequency) = judge(&answer , default_config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
//                     used_word_frequency = frequency;
//                     _flag = false;
//                     count_success_loop += success;
//                     count_played += 1;
//                     if success != 0{
//                         count_success += 1;
//                     }
//                     success_judge(w_mode , success, answer);

//                     if default_config.stats.unwrap(){
//                         print_state(count_success , count_played , count_success_loop , used_word_frequency.clone(), matches.contains_id("state") , json.clone());
//                     }
//                 }
//                 None => {
//                     loop{
//                         if count_played != 0{                 //Check if player want another round
//                             let mut _flag = true;
//                             let mut line = String::new();
//                             io::stdin().read_line(&mut line)?;
//                             if line == "N\n" || line == "n\n"{
//                                 break;
//                             }
//                         }
//                         let mut line = String::new();
//                         io::stdin().read_line(&mut line)?;
//                         line.pop();
//                         if !acceptable_set.contains(&line.as_str()){
//                             panic!("INVALID");
//                         }
//                         let (success , _guess ,frequency) = judge(&line , default_config.difficult.unwrap(), used_word_frequency.clone() , &acceptable_set);
//                         used_word_frequency = frequency;
//                         _flag = false;
//                         w_mode = true;
//                         answer = line;

//                         success_judge(w_mode , success, answer.clone());
//                         count_played += 1;
//                         if success != 0{
//                             count_success += 1;
//                             count_success_loop += success;
//                         }

//                         if default_config.stats.unwrap(){
//                             print_state(count_success , count_played , count_success_loop , used_word_frequency.clone() , matches.contains_id("state") , json.clone());
//                         }
//                     }
//                 }
//             }
//         }
//         if default_config.random.unwrap() && _flag{
//             loop{
//                 let mut line = String::new();
//                 if matches.contains_id("final-set"){
//                     line = get_useable_word_file(default_config.day.unwrap(), default_config.seed.unwrap().try_into().unwrap(), default_config.final_set.clone().unwrap().as_str());
//                 }
//                 else{
//                     line = get_useable_word_default(default_config.day.unwrap(), default_config.seed.unwrap().try_into().unwrap());
//                 }
//                 if answer_used.contains(&line){                     //Check if the word has been used
//                     continue;
//                 }
            
//                 let (success , gusses , frequency) = judge(&line , default_config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
//                 used_word_frequency = frequency;
//                 _flag = false;
//                 answer = line;
//                 success_judge(w_mode , success, answer.clone());
//                 answer_used.push(answer.clone());
//                 count_played += 1;
//                 if success != 0{
//                     count_success += 1;
//                     count_success_loop += success;            
//                     }
//                 if default_config.stats.unwrap(){
//                     print_state(count_success , count_played , count_success_loop , used_word_frequency.clone() , matches.contains_id("state") , json.clone());
//                 }
//                 if matches.contains_id("state"){
//                     let _ = state_to_json(matches.get_one::<String>("state").unwrap().clone() , answer.clone() , gusses.clone());
//                 }
                    
//                 if count_played != 0{                 //Check if player want another round
//                     let mut _flag = true;
//                     let mut line = String::new();
//                     io::stdin().read_line(&mut line)?;
//                     if line == "N\n" || line == "n\n" || line == "N" || line == "n"{
//                         break;
//                     }
//                 }

//                 default_config.day = Some(default_config.day.unwrap() + 1);
//             }
//         }
//         if _flag{                                    //default mode
//             loop{
//                     if count_played != 0{                 //Check if player want another round
//                         let mut _flag = true;
//                         let mut line = String::new();
//                         io::stdin().read_line(&mut line)?;
//                         if line == "N\n" || line == "n\n"{
//                             break;
//                         }
//                     }
//                     let mut line = String::new();
//                     io::stdin().read_line(&mut line)?;
//                     line.pop();
//                     if !acceptable_set.contains(&line.as_str()){
//                         panic!("INVALID");
//                     }
//                     let (success , _gusses ,frequency) = judge(&line , default_config.difficult.unwrap() , used_word_frequency.clone() , &acceptable_set);
//                     used_word_frequency = frequency;
//                     _flag = false;
//                     answer = line;
//                     success_judge(w_mode , success, answer);
//                     count_played += 1;
//                     if success != 0{
//                         count_success += 1;
//                         count_success_loop += success;
//                     }

//                     if default_config.stats.unwrap(){
//                         print_state(count_success , count_played , count_success_loop , used_word_frequency.clone() , matches.contains_id("state") , json.clone());
//                     }
//             }
//             }

       
//     }
//         Ok(())
// }


pub fn _dmode_vavid_check(str : &str , input : &String , result : &String) -> bool {
    let mut yellow = Vec::new();

    if str == ""{
        return true;
    }
    for ((c_default , c_input ), c_result) in str.chars().zip(input.chars()).zip(result.chars()) {
        if c_result == 'G' && c_default != c_input {
            return false;
        }
        if c_result == 'Y'{
            yellow.push(c_default);
        }
    }

    for c in yellow {
        if !input.contains(c) {
            return false;
        }
    }

    true
}

pub fn hash_map_sort(map: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut vec: Vec<(String, i32)> = map.into_iter().collect();
    vec.sort_by(|a, b| {
        // 先按值降序排序
        let value_cmp = b.1.cmp(&a.1);
        if value_cmp == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            value_cmp
        }
    });
    let mut result = HashMap::new();
    for value in vec.iter_mut(){
        result.insert(value.0.clone(), value.1);
    };
    result
}

use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use crate::builtin_words::select;

pub fn get_useable_word_default(day : usize, seed : u64) -> String{                    //Get the word from the default set
    let mut rng = StdRng::seed_from_u64(seed);
    let mut vec = select::FINAL.to_vec();
    vec.shuffle(&mut rng);
    vec[day-1].to_string()
}



pub fn fix_string_by_index(input : &str , index : usize , c : char) -> String{
    let mut result = String::new();
    for i in 0..input.len(){
        if i == index{
            result.push(c);
        }
        else{
            result.push(input.chars().nth(i).unwrap());
        }
    }
    result
}
