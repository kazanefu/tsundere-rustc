use colored::*;
use rand::prelude::*;
use regex::Regex;
use std::env;
use std::process::Command;

/// 英語エラーメッセージを日本語ツンデレ翻訳する
fn tsundere_translate(msg: &str) -> String {
    let mut result = String::new();

    // 辞書マップ
    let translations = [
        (r"not found", "そんなの知らないんだから！"),
        (r"mismatched types", "型が違うってば！ちゃんとしてよ！"),
        (r"expected", "期待してたのに…（違う型じゃない…）"),
        (r"borrowed", "また借用のことで怒られたいの？"),
        (r"cannot find", "見つかんないし！ちゃんと定義してよね！"),
        (r"use of moved value", "もうその変数、いなくなっちゃったんだから！"),
        (r"missing lifetime specifier", "ライフタイムが足りないのよ！"),
        (r"the trait bound", "トレイトの制約が合ってないわよ！"),
        (r"does not implement", "実装してないじゃない！"),
        (r"unresolved import", "インポートミスってるわよ！"),
    ];

    for (pattern, jp) in translations.iter() {
        let re = Regex::new(pattern).unwrap();
        if re.is_match(msg) {
            result.push_str(jp);
            return result;
        }
    }

    result.push_str("なにそれ…意味わかんない…（でも応援してあげる）");
    result
}

/// ツンデレ語尾をランダムに付ける
fn tsundere_suffix() -> &'static str {
    let suffixes = [
        "（べ、別に怒ってないんだからね！）",
        "（ちょっとは頑張ったじゃない…）",
        "（あんたのことなんか心配してないんだから！）",
        "（次はうまくいく…かもね///）",
        "（もう…仕方ないなぁ…）",
    ];
    let mut rng = rand::rng();
    suffixes.choose(&mut rng).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("使い方: tsundere-rustc <ファイル名>");
        std::process::exit(1);
    }

    let file = &args[1];
    let output = Command::new("rustc")
        .arg(file)
        .output()
        .expect("rustcの実行に失敗しました");

    if output.status.success() {
        println!("{}", "/// べ、別に成功して嬉しいわけじゃないんだからね！".green());
        return;
    }

    println!("{}", "💢 コンパイルエラー発生！".red().bold());
    let stderr = String::from_utf8_lossy(&output.stderr);

    for line in stderr.lines() {
        if line.starts_with("error") || line.contains("error[E") {
            println!("{}", format!("🦀 {}", tsundere_translate(line)).yellow());
            println!("{}", tsundere_suffix().italic());
        } else if line.starts_with("note") {
            println!("{}", format!("💬 ヒント: {}", line).blue());
        } else if line.starts_with("help") {
            println!("{}", format!("💡 助言: {}", line).cyan());
        }
    }
}
