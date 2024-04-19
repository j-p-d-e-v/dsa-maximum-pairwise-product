use std::io;
use std::collections::HashMap;

fn main(){
    let mut n_str: String = String::new();
    match io::stdin().read_line(&mut n_str) {
        Ok(_) => {
            let n: u64 = n_str.trim().parse::<u64>().expect("Expecting a non-negative integer number.");
            let mut numbers_str: String = String::new();
            match io::stdin().read_line(&mut numbers_str) {
                Ok(_) => {
                    let numbers: Vec<&str> = numbers_str.trim().split(" ").collect::<Vec<&str>>();
                    let mut pairs: HashMap<u8,(i32,u64)> = HashMap::new();
                    pairs.insert(1,(-1,0));
                    pairs.insert(2,(-1,0));
                    
                    if numbers.len() as u64 != n {
                        panic!("Total numbers length not equal to the declared length. Got {}/{}",numbers.len(),n);
                    }
                    for pair_num in [1,2] {
                        for (i,value) in numbers.clone().into_iter().enumerate() {
                            let pair: &(i32,u64) = pairs.get(&pair_num).unwrap();
                            let number = value.parse::<u64>().expect("Expecting a number.");
                            match pair_num {
                                1 => {
                                    if number >= pair.1 {
                                        pairs.insert(pair_num,(i as i32,number));
                                    }
                                },
                                _ => {
                                    let first_pair = pairs.get(&1).unwrap();                                
                                    if number >= pair.1 && first_pair.0 != i as i32  {
                                        pairs.insert(pair_num,(i as i32,number));
                                    }
                                }
                            }
                        }
                    }
                    println!("{}", pairs.get(&1).unwrap().1 * pairs.get(&2).unwrap().1);
                },
                Err(e) => panic!("Error: {}",e),
            }
        },
        Err(e) => panic!("Error: {}",e),
    }
}
