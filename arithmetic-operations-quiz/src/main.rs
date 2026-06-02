fn main() {
    let mut num_of_correct = 0;
    while num_of_correct < 3 {
        let quiz_mode = rand::random_range(1..=2);

        match quiz_mode {
            1 => loop {
                let op1 = rand::random_range(0..100);
                let op2 = rand::random_range(0..100);
                let ans1 = op1 + op2;

                println!("{} + {} = ??", op1, op2);
                println!("??の値を入力してください。");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();
                let ans_input = ans_input.trim().parse::<i32>().ok();
                match ans_input {
                    Some(ans_input) => {
                        if ans_input == op1 + op2 {
                            println!("正解");
                            num_of_correct += 1;
                            break;
                        } else {
                            println!("不正解");
                            println!("正解は： {}", ans1);
                        }
                    }
                    None => {
                        println!("範囲{} ~ {}の数値を入力してください。", i32::MIN, i32::MAX);
                        continue;
                    }
                }
            },

            2 => loop {
                let op3 = rand::random_range(0..100);
                let op4 = rand::random_range(0..100);
                let ans2 = op3 - op4;

                println!("{} - {} = ??", op3, op4);
                println!("??の値を入力してください。");
                let mut ans_input = String::new();
                std::io::stdin().read_line(&mut ans_input).unwrap();
                let ans_input = ans_input.trim().parse::<i32>().ok();
                match ans_input {
                    Some(ans_input) => {
                        if dbg!(ans_input == op3 - op4) {
                            println!("正解");
                            num_of_correct += 1;
                            break;
                        } else {
                            println!("不正解");
                            println!("正解は： {}", ans2);
                        }
                    }
                    None => {
                        println!("範囲{} ~ {}の数値を入力してください。", i32::MIN, i32::MAX);
                        continue;
                    }
                }
            },

            _ => unreachable!(),
        }
    }
}
