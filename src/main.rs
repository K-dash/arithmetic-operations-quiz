use rand::Rng;

fn main() {
    let mut num_of_correct = 0;
    // 3回正解したら終了
    while num_of_correct < 3 {
        // quiz_mode をランダムに1か2に決める
        let quiz_mode = rand::thread_rng().gen_range(1..=2);
        match quiz_mode {
            1 =>  loop {
                // quiz_mode が 1 の場合は 加算クイズ
                let op1 = rand::thread_rng().gen_range(1..100);
                let op2 = rand::thread_rng().gen_range(1..100);

                println!("{} + {} = ??", op1, op2);
                println!("?? の値を入力してください:");
                let mut ans_input = String::new(); // ユーザーからの回答を保持する変数
                                                   // 標準入力から1行取得して、ans_input に代入する
                std::io::stdin().read_line(&mut ans_input).unwrap();

                // ans_inputからtrimで改行を取り除き、parseで整数(i32)に変換する
                // こっちはResult型で実装する場合
                match ans_input.trim().parse::<i32>() {
                    Ok(num) => {
                        if num == op1 + op2 {
                            println!("正解!");
                            num_of_correct += 1; // 正解数をインクリメント
                            break;
                        } else {
                            println!("不正解!");
                        }
                    }
                    Err(_) => {
                        println!("数値を入力してください");
                        continue;
                    }
                }
            }
            2 => loop {  // quiz_mode が 2 の場合は 減算クイズ
                let op1 = rand::thread_rng().gen_range(1..100);
                let op2 = rand::thread_rng().gen_range(1..100);

                println!("{} - {} = ??", op1, op2);
                println!("?? の値を入力してください:");
                let mut ans_input = String::new(); // ユーザーからの回答を保持する変数
                                                   // 標準入力から1行取得して、ans_input に代入する
                std::io::stdin().read_line(&mut ans_input).unwrap();

                // ans_inputからtrimで改行を取り除き、parseで整数(i32)に変換する
                // こっちはOption型で実装する場合
                match ans_input.trim().parse::<i32>().ok() {
                    Some(num) => {
                        if num == op1 - op2 {
                            println!("正解!");
                            num_of_correct += 1; // 正解数をインクリメント
                            break;
                        } else {
                            println!("不正解!");
                        }
                    }
                    None => {
                        println!("数値を入力してください");
                        continue;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    println!("クリアしました!");
}
