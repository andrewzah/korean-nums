mod number;
mod counter;

pub fn calculate(numbers: Vec<String>) -> String {
    let len = numbers.len();
    let mut output = String::from("");
    let mut output2 = String::from("");

    let mut iter = numbers.iter().rev().peekable();
    let mut i = len;
    while let Some(num) = iter.peek() {
        let modulo = i % 4;

        let korean_num = number::Number::from_str(num).unwrap();
        output2.push_str(korean_num.to_str_sino().unwrap());

        if i > 0 {
            if let Some(next) = iter.peek() {
                if next.as_str() != "0" && num != "0" {
                    let position_counter = counter::Position::from_usize(modulo).unwrap();
                    output2.push_str(position_counter.as_str());
                }
            }
        }

        if modulo == 0 && i != 0 {
            if let Some(next) = iter.peek() {
                println!("next exists");
            }
        }

        i -= 1;
    }
    output2


    //for (i, number) in numbers.iter().enumerate() {
        //let idx = len - i - 1;

        //let modulo = idx % 4;
        //let num = number::Number::from_str(number).unwrap();

        //output.push_str(num.to_str_sino().unwrap());

        //if idx > 0 {
            //let next = numbers.get(idx-1).unwrap().as_str();
            //if  next != "0" && number != "0" {
                //let position_counter = counter::Position::from_usize(modulo).unwrap();
                //output.push_str(position_counter.as_str());
            //}
        //}

        // problem: checking ahead for blocks
        // need to grab as big of chunks ahead, larger than 4
        // terminate if len of zeroes == len of idx->string end
        //if modulo == 0 && idx != 0 {
            //let mut zeroes = String::new();
            //let mut iter = numbers[..idx].iter();
            //while let Some(zero) = iter.next() {
                //if zero == "0" {
                    //zeroes.push_str(zero);
                //}
            //}
            //println!("numbers_len: {}, zeroes_len: {}, zeroes: {}", len, zeroes.len(), zeroes);

            //let slice = numbers[(idx-4)..idx].join("");
            //println!("current_index: {}, slice: {}", idx, &slice);
            //if  slice == "0000" {
                //println!("current_index: {}", idx);
                //let block_counter = counter::Block::from_index(idx).unwrap();
                //output.push_str(block_counter.as_str());
                //output.push_str(" ");
            //}
        //}
        //println!("curr_output: {}", &output);
    //}

    //output
}

pub fn str_to_vec_string(s: &str) -> Vec<String> {
    s
        .chars()
        .map(|c| c.to_string())
        .collect()
}

pub fn i32_to_vec_string(num: i32) -> Vec<String> {
    str_to_vec_string(&num.to_string())
}

pub fn u32_to_vec_string(num: u32) -> Vec<String> {
    str_to_vec_string(&num.to_string())
}

