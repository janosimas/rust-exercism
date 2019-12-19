pub fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    let size: u32 = str_num.len() as u32;

    let sum = str_num
        .chars()
        .map(|val| val.to_digit(10).unwrap().pow(size))
        .sum();

    num == sum
}
