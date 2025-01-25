use std::collections::HashMap;

fn mean(v: &Vec<i32>)-> f64 {
    let mut sum=0;
    let mut index=0;

    for i in v{
        sum= sum+i;
        //print!("{}",i);
        index +=1;
    }
    sum as f64/ index as f64
}

fn median(mut v: Vec<i32>) -> i32{
    v.sort();
    let v_len=v.len();
    let v_index = (v_len/2)+((v_len/2)%2);
    let mid: Option<&i32> = v.get(v_index);
    match mid {
        Some(mid) => return * mid,
        None => {println!("There is no element.");
        return 0}
    }
}

fn mode(v: &Vec<i32>)->i32{
    let mut mode_num=0;
    let mut max_count=0;

    let mut occurrences = HashMap::new();
    for &i in v{
        let count = occurrences.entry(i).or_insert(0);
        *count += 1;
        //println!("{:?}",occurrences);
    }
    for (key, value) in occurrences {
        if value > max_count {
            max_count = value;
            mode_num = key;
        }
    }
    mode_num
}

fn main() {
    let mut v_list = Vec::new();
    v_list.push(5);
    v_list.push(3);
    v_list.push(5);
    v_list.push(9);
    v_list.push(5);
    v_list.push(9);
    v_list.push(1);
    v_list.push(5);
    v_list.push(2);
    v_list.push(5);    
    
    println!("The mean is {}",mean(&v_list));
    println!("The median is {}",median(v_list.clone()));
    println!("The mode is {}",mode(&v_list));
}
