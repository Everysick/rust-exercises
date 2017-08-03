#![allow(dead_code)]
fn main() {
    let (word_a, word_b) = words();
    let message = format!("{}{}！", word_a, word_b);
    output(message);
    //output(message);
}

fn words() -> (String, String) {
    (format!("こんにちは"), format!("世界"))
}

fn output(text: String) {
    // let kanji_only = remove_hiragana(text.clone()); // copyしろと暗に言っている気がした
    // let kanji_only = remove_hiragana(&text); // 参照を渡せばいい? 所有権だけなのでだめそう
    let (kanji_only, text) = remove_hiragana(text); // 所有権を戻す
    println!("{}", kanji_only);
    /*
    ゴール2：次の行をアンコメントすると何がおきるでしょうか？
    これをコンパイルを通すにはどうすれば良いでしょうか？
    */
    println!(
        "ひらがなを抜き取ると：{:?} → {:?}",
        text,
        kanji_only
    );

    /*
    ゴール3：データをコピーせずにコンパイルを通すにはどおすれば良いでしょうか？
    所有権の移動のみを使って解決してください
    */
}

fn remove_hiragana(text: String) -> (String, String) {
    /*
     ゴール1：コンパイルを通すには何を変更すれば良いでしょうか
    */
    let mut result = String::new(); // add mut
    for c in text.chars() {
        if c < 'ぁ' || 'ん' < c {
            result.push(c);
        };
    }
    (result, text) // 所有権を戻す
}
