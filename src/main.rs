fn main() {
    println!("1 + 1 = ??");
    println!("?? の値を入力してください:");
    let mut ans_input = String::new(); // ユーザーからの回答を保持する変数
                                       // 標準入力から1行取得して、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    // ans_inputからtrimで改行を取り除き、parseで整数(u32)に変換する
    let ans_input = ans_input.trim().parse::<u32>().unwrap();

    dbg!(ans_input); // 実行後に入力された値を確認

    if ans_input == 1 + 1 {
        println!("正解!");
    } else {
        println!("不正解!");
    }

    println!("1 - 4 = ??");
    println!("?? の値を入力してください:");
    let mut ans_input = String::new(); // ユーザーからの回答を保持する変数
                                       // 標準入力から1行取得して、ans_input に代入する
    std::io::stdin().read_line(&mut ans_input).unwrap();

    let ans_input = ans_input.trim().parse::<isize>().unwrap();

    dbg!(ans_input); // 実行後に入力された値を確認

    if ans_input == 1 - 4 {
        println!("正解!");
    } else {
        println!("不正解!");
    }

    println!("i32 が扱えるデータ範囲: {} ~ {}", isize::MIN, isize::MAX);
    println!("u32 が扱えるデータ範囲: {} ~ {}", usize::MIN, usize::MAX);
}
