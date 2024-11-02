use rand::Rng;

fn main() {
    let op1 = rand::thread_rng().gen_range(1..100);
    let op2 = rand::thread_rng().gen_range(1..100);

    println!("{} + {} = ??", op1, op2);
    println!("?? の値を入力してください:");
    let mut ans_input = String::new(); // ユーザーからの回答を保持する変数
                                       // 標準入力から1行取得して、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_inputからtrimで改行を取り除き、parseで整数(i32)に変換する
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input);

    if ans_input == op1 + op2 {
        println!("正解!");
    } else {
        println!("不正解!");
    }


    let op1 = rand::thread_rng().gen_range(1..100);
    let op2 = rand::thread_rng().gen_range(1..100);

    println!("{} - {} = ??", op1, op2);
    println!("?? の値を入力してください:");
    let mut ans_input = String::new(); // ユーザーからの回答を保持する変数
                                       // 標準入力から1行取得して、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_inputからtrimで改行を取り除き、parseで整数(i32)に変換する
    let ans_input = ans_input.trim().parse::<i32>().unwrap();

    dbg!(ans_input);

    if ans_input == op1 - op2 {
        println!("正解!");
    } else {
        println!("不正解!");
    }
}
