fn main() {
    let x = 1;
    let x = x + 1;

    println!("{}", x); // 2

    {
        // このコードは、変数の再宣言なので型が違いますが、コンパイルエラーはおきません。
        // &str型
        let str = "Hello World!";
        println!("{}", str); // Hello World!
    
        // usize型
        let str = str.len();
        println!("{}", str); // Hello World!の文字数
    }

    {
        // このコードは、変数strがmutableですが、最初に型が&strであるため、数値を再代入することはできません。
        // 従って、コンパイルエラーが発生します。
        let mut str = "Hello World!";
        println!("{}", str); // Hello World!

        // str = str.len(); // コンパイルエラー！
    }
    // このコメントはコミットするだけのコメント。Githubの草を継続するんだ。
    // 一見何の意味もないことかもしれないが、それは違う。
    // Githubを見た時に、緑色に染まっているとモチベーションが上がる。
    // 毎日コミットすることが大事。
    // いえーい！
    // さいこーーー！！
    // ・・・
    // 

    
}
