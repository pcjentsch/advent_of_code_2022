use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let problem = args[1].as_str();
    let ans = match problem {
        "1" => advent_1("input_1a"),
        "2a" => advent_2a("input_2a"),
        "2b" => advent_2b("input_2a"),
        _ => 0,
    };
    dbg!(ans);
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


}

