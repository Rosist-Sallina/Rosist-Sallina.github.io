pub mod crate_judge{
    use std::collections::HashMap;
    use crate::resouces::fix_string_by_index;
    pub fn judge(input:Vec<String> , str:&str , mut char_color: HashMap<char , char>) -> (String ,String , String , HashMap<char , char>){                         //All judge function
        let mut default_map = HashMap::new();
        let input = input.join("");
    
        for c in str.chars() {
            let count = default_map.entry(c).or_insert(0);
            *count += 1;
        } //Generate a map for word(to be guessed)'s color  # default
        
            let mut _i = 0;
            let mut result = String::new();
            // if flag && !_dmode_vavid_check(&last, &input, &_result){
            //     println!("INVALID");
            //     continue;
            // }
            // used_word_frequency.entry(input.clone()).or_insert(0);
            // if used_word_frequency.contains_key(&input){
            //     let count = used_word_frequency.entry(input.clone()).or_insert(0);
            //     *count += 1;
            // }
            // gusses.push(input.clone().to_uppercase());
    
            let mut map_used = HashMap::new();  //how many (char,num) char we have used
            let mut i = 0;
    
            for c in input.chars(){
                if input.chars().nth(i) == str.chars().nth(i){
                    let count = map_used.entry(c).or_insert(0);
                    *count += 1;
                    result.push('G');
                    char_color.insert(input.chars().nth(i).unwrap(), 'G');
                    i += 1;
                    continue;
                }
                else {
                    result.push(' ');
                    i += 1;
                }
            }
            i = 0;
            for c in input.chars() {
                if result.chars().nth(i).unwrap() == 'G' {
                    i += 1;
                    continue;
                }
                if default_map.contains_key(&c) && default_map.get(&c) > map_used.get(&c) {    //available char still in the word
                    let count = map_used.entry(c).or_insert(0);
                    *count += 1;
                    result = fix_string_by_index(&result , i , 'Y');
                    if char_color.contains_key(&c) && *char_color.get(&c).unwrap() == 'G' {
                        i += 1;
                        continue;
                    }
                    else{
                        char_color.insert(input.chars().nth(i).unwrap(), 'Y');
                    }
                    i += 1;
                    continue;
                } 
                else {
                    let count = map_used.entry(c).or_insert(0);
                    *count += 1;
                    result = fix_string_by_index(&result , i , 'R');
                    if char_color.contains_key(&c) && (*char_color.get(&c).unwrap() == 'G' || *char_color.get(&c).unwrap() == 'Y') {
                        i += 1;
                        continue;
                    }
                    else{
                        char_color.insert(input.chars().nth(i).unwrap(), 'R');
                    }
                    i += 1;
                    continue;
                }        
            }
            let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let mut alphabet_result = String::new();
            for c in alphabet.chars() {
                if !char_color.contains_key(&c) {
                    alphabet_result.push('X');
                }
                else{
                    alphabet_result.push(*char_color.get(&c).unwrap());
                }
            }
        return (result, input , alphabet_result , char_color);
    }
}