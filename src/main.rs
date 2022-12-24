use std::fs;
const FILENAME: &str="./input.txt";

fn main() {
    let input_raw =fs::read_to_string(FILENAME).unwrap();
    let numbe:Vec<&str>= input_raw.split("\n").collect();
    let mut total: Vec<i32> =Vec::new();
    let mut elfcalories: Vec<i32> =Vec::new();
    let mut sume=0;
    for line in  numbe{
        println!("{}",line.len());
        if line.len()<1 {
            for i in 0 .. elfcalories.len(){
                sume = sume + elfcalories[i];
            }
            total.push(sume);
            elfcalories=Vec::new();
            sume=0;
        }
        else
        {
            elfcalories.push(line.parse::<i32>().unwrap());
        }
    }
    total.sort();
    let re= total[total.len()-1];
    println!("{}",re);
    let mut rsum=0;
    for i in 1..4{
        rsum=rsum+total[total.len()-i];
    }
    println!("{}",rsum);
    
    
}
