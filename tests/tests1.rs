
fn calculate_prince(num: u32)-> u32{
if num >40 {
    num
}else {
    num * 2
}
}



#[test]

fn verify_test(){
    let prince1 = calculate_prince(55);
    let prince2 = calculate_prince(80);
}