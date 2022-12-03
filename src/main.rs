use std::env;
use std::fs;
use std::ops::Div;
fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args[1].as_str();
    let ans = match problem {
        "1" => advent_1("input_1a"),
        "2a" => advent_2a("input_2a"),
        "2b" => advent_2b("input_2a"),
        "3" => advent_3a("input_3"),
        _ => 0,
    };
    println!("{}",ans);
}

fn advent_1(path: &str) -> usize{
    let contents = fs::read_to_string(path).expect("file not found");
    let mut elf_cals:Vec<usize> =vec!();
    elf_cals.push(0);
    for line in contents.split("\n"){
        if line.is_empty(){
            elf_cals.push(0);
        }
        else
        {   
            let l = elf_cals.len();
            elf_cals[l-1] += str::parse::<usize>(line).expect("cannot parse");
        }
    }
    elf_cals.sort();
    let top_three = &elf_cals[elf_cals.len()-3..elf_cals.len()];
    return top_three.iter().sum::<usize>();
}

fn advent_2a(path: &str) -> usize{
    let contents = fs::read_to_string(path).expect("file not found");
    let mut score: usize = 0;
    for line in contents.split("\n"){
        let line_split = line.split(" ").collect::<Vec<&str>>();
        let (mv_opp, mv_me) = (line_split[0], line_split[1]);
        score += match mv_me { //ok I know there is a better way but i dont care
            "X" => 1 + match mv_opp {
                "C" => 6,
                "A" => 3,
                "B" => 0,
                _ => panic!("symbol not recognized")
            },
            "Y" => 2 + match mv_opp {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => panic!("symbol not recognized")
            },
            "Z" => 3 + match mv_opp {
                "B" => 6,
                "C" => 3,
                "A" => 0,
                _ => panic!("symbol not recognized")
            },
            _ => panic!("symbol not recognized")
        }
    }
    return score;
}


fn advent_2b(path: &str) -> usize{
    let contents = fs::read_to_string(path).expect("file not found");
    let mut score: usize = 0;
    for line in contents.split("\n"){
        let line_split = line.split(" ").collect::<Vec<&str>>();
        let (mv_opp, mv_me) = (line_split[0], line_split[1]);
        score += match mv_me { //ok I know there is a better way but i dont care
            "X" => 0 + match mv_opp {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => panic!("symbol not recognized")
            },
            "Y" => 3 + match mv_opp {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => panic!("symbol not recognized")
            },
            "Z" => 6 + match mv_opp {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => panic!("symbol not recognized")
            },
            _ => panic!("symbol not recognized")
        }
    }
    return score;
}

fn to_priority(items: &str) -> Vec<u8>{
    return items.as_bytes().into_iter().map(|&x| 
        if x>=65 && x <= 91{
            x - 38
        }
        else{
            x - 96
        }
    ).collect();
}



fn advent_3a(path: &str)->usize{
    let contents = fs::read_to_string(path).expect("file not found");
    let mut total_priority = 0;
    for line in contents.split("\n"){
        let compartment_size = line.len().div(2);
        let left_compartment = to_priority(&line[0..compartment_size]);
        let right_compartment = to_priority(&line[compartment_size..line.len()]);
        let mut left_compartment_flags:u64 = 0;
        let mut right_compartment_flags:u64 = 0;
        for item in left_compartment{
            left_compartment_flags = left_compartment_flags | (1 << item);
        }
        for item in right_compartment{
            right_compartment_flags = right_compartment_flags | (1 << item);
        }
        let shared = ((left_compartment_flags & right_compartment_flags) as f64).log2() as usize;
        total_priority += shared;
    }
    return total_priority;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advent_2()
    {
        assert_eq!(advent_2a("test_2a"),15);
    }
    #[test]
    fn test_advent_2b()
    {
        assert_eq!(advent_2b("test_2a"),12);
    }
    #[test]
    fn test_advent_3()
    {
        assert_eq!(advent_3a("test_3"),157);
    }


}

