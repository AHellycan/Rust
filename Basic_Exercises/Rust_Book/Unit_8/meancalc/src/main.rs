fn mean(v: Vec<i32>)-> f64 {
    let mut sum=0;
    let mut index=0;

    for i in v{
        sum= sum+i;
        //print!("{}",i);
        index +=1;
    }
    sum as f64/ index as f64
}

fn main() {
    let mut v_list = Vec::new();
    v_list.push(5);
    v_list.push(3);
    v_list.push(4);
    v_list.push(9);    
    
    println!("The mean is {}",mean(v_list));
}
