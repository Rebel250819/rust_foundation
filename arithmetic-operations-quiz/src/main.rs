fn main() {
    let op1 = rand::random_range(0..100);
    let op2 = rand::random_range(0..100);
    let ans1 = op1 + op2;

    println!("{} + {} = ??", op1, op2);
    println!("??の値を入力してください。");
    let mut ans_input = String::new();
    std::io::stdin().read_line(&mut ans_input).unwrap();
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);

    if dbg!(ans_input == op1 + op2) {
        println!("正解");
    } else {
        println!("不正解");
        println!("正解は： {}", ans1);
    }

    let op3 = rand::random_range(0..100);
    let op4 = rand::random_range(0..100);
    let ans2 = op3 - op4;

    println!("{} - {} = ??", op3, op4);
    println!("??の値を入力してください。");
    let mut ans_input = String::new();
    std::io::stdin().read_line(&mut ans_input).unwrap();
    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);
    if dbg!(ans_input == op3 - op4) {
        println!("正解")
    } else {
        println!("不正解");
        println!("正解は： {}", ans2);
    }

    println!("i32 が扱えるデータ範囲 {} ~ {}", i32::MIN, i32::MAX);
    println!("u32 が扱えるデータ範囲 {} ~ {}", u32::MIN, u32::MAX);
}
