// The Rust Programming Language を徹夜で読んでやれるだけ試してみた
// https://doc.rust-jp.rs/book/second-edition/foreword.html

// rustup doc で、ドキュメントが立ち上がる
// cargo doc --open で自分で作ったやつのドキュメントが立ち上がる

fn main() {
    let a = 1; // 型推論
    let b :i32 = 1; // 明示

    println!("{}", a);
    println!("{}", b);

    // a = 30; // 変数は immutable なので変更できない

    let a = a + b; // a + b を 新たに宣言した a に入れることは出来る

    println!("{}", a);

    let mut a = 1; // 変更可能な変数には mut を付ける

    a *= 10;

    println!("{}", a);

    let s = String::from("abc"); // 文字列

    println!("{}",s);

    let s = String::from("へろー");

    println!("{}",&s[0..3]); // 「へ」 よろしくない

    for c in s.chars() { // 文字単位
        println!("{}",c)
    }
    for b in s.bytes() { // バイト単位
        println!("{}",b)
    }

    let v = s.chars();
    println!("{:?}", v); // "{:?}" "{:#?}" でdumpできる。{:#?}の方が、見た目よろしくdumpする

    for c in v {
        println!("{}", c)
    }

    // ここで、
    // println!("{:?}", v)
    // は、for で v が move されているのでエラー

    let mut s = String::from("はろー");

    s.push_str(", わーるど"); // 文字のの連結

    let s2 = String::from("! あいうえお");
    let s = s + &s2; // + も使えるけど、右側には参照を渡す

    println!("{}", s);

    let v = s.chars().nth(0).expect("「は」が取れるはず"); // expect は、失敗した際に panicする時に出す文字列
    println!("{}", v);

    // オーバーすると
    // let v = s.chars().nth(100).expect("ぱにくる");

    let num = "123".to_string().parse::<i32>(); // "文字".to_string() も String::from("文字") もどっちでも良い
    let num2 = String::from("123").parse::<i32>();

    // if は普通
    if num == num2 {
        println!("同じ")
    }

    let mut cnt = 0;

    let n = loop { // 無限 loop
        cnt += 1;
        println!("ループ!: {}回目", cnt);
        if cnt == 10 {
            break cnt // break を loop の戻り値として返せる
        }
    };

    println!("breakの戻り!:{}", n);


    let r = s.chars().nth(100); // Option を返してくる
    let v = match r {
        Some(_) => s, // なんか返ってきたら
        None => String::from("NULL"), // オーバーしたら"NULL"って返す
    };

    println!("{}", v); // "NULL"って出る

    let s = pass_string(v);
    println!("{}", s);

    // println!("{}", v); // pass_string で moveされているので死ぬ

    let v = pass_ref_string(&s);
    println!("{}", s); // 参照渡しなので死なない
    println!("{}", v);

    let v = String::from("えええ");
    println!("{}", modify_string(v)); // modify_string 側で mutを付けると変更できる

    if s == "NULL" { // v は modify_string で moveされているので使えない
        println!("True");
    } else {
        println!("False");
    }

    // enum を使う
    let rice = Breakfast::Rice;
    let bread = Breakfast::Bread;

    let breakfast = [rice, bread]; // 配列

    for r in breakfast.iter() {
        // enum をパターンマッチ
        match r {
            Breakfast::Rice => {
                println!("米がたべたい")
            },
            Breakfast::Bread => {
                println!("パンがたべたい")
            },
        }
    }

    // 文字列を取る enum を使う
    let b = Breakfast2::Kind(String::from("炊き込みご飯"));
    let c = Breakfast2::Calorie(1000);

    // Breakfast2 は std::fmt::Display を実装していないので、"{}"は使えない
    // #[derive(Debug)] しているので、"{:?}" は使える
    println!("朝食の種類:{:?}", b);
    println!("カロリー:{:?}", c);

    // Breackfast3
    let b = Breakfast3::Kind(String::from("炊き込みご飯"));
    let c = Breakfast3::Calorie(1000);

    // Breackfast3 は std::fmt::Display を実装しているので、"{}"が使える
    println!("朝食の種類:{}", b);
    println!("カロリー:{}", c);

    // モジュール person。new 関数を construct として使っている(が、別に new が特別なワードではない)
    let p = person::Person::new(String::from("Atsushi Kato"), 173, 65, String::from("fist"));
    println!("名前: {}", p.name); // name は pub なのでアクセス可能
    println!("身長: {}", p.height());
    println!("体重: {}", p.weight());

    // モジュール animal. new 関数を construct として使っている(が、別に new が特別なワードではない)
    let a = animal::Animal::new(String::from("Monkey"), 133, 45, String::from("fang"));
    println!("名前: {}", a.name); // name は pub なのでアクセス可能
    println!("身長: {}", a.height());
    println!("体重: {}", a.weight());

    // walk メソッドを使ってみる。これは trait(go の interface的なもの) Creature を実装している
    // trait では default を定義できるので、animal では実装していない
    p.walk();
    a.walk();

    // whtat_weapon_has は、Creature の trait を満たしていれば、使うことが出来る
    println!("{}",what_weapon_has(p));
    println!("{}",what_weapon_has(a));

    // ベクタ
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v[0] = 100;
    println!("{:#?}", v);

    let mut v = vec![0 ; 10]; // 0 ; 10 で 0 を 10こ作る
    v[0] = 101;
    v.push(1);
    v.push(2);
    println!("{:#?}", v);

    let r = v.get(100);
    let r = match r {
        Some(_) => n,
        None => { // 式も書けるよ
            println!("取得失敗");
            0
        },
    };
    println!("戻り:{}", r);

    // HashMap
    use std::collections::HashMap;

    let mut hash = HashMap::new();

    let n1 = 10;
    let n2 = 20;

    hash.insert("あいうえお".to_string(), n1);
    hash.insert("かきくけこ".to_string(), n2);

    println!("{:#?}", hash);
    println!("{:#?}", hash.get(&"あいうえお".to_string())); // get するときはキーの名前は参照で渡す必要がある

    // ハッシュの値を変更
    // あれば、値を変更する and_modify は無名関数(ココでは、|v| *v = 10000)が引数(複数行の時は、 {} を使う)を取る
    hash.entry("あいうえお".to_string()).and_modify(|v| *v=10000);
    // なければ、値を入れる or_insert はそのまま値を取れる
    hash.entry("ほげ".to_string()).or_insert(100);

    println!("{:#?}", hash);

}

fn pass_string (s: String) -> String {
    s
}

fn pass_ref_string (s: &String) -> &String {
    &s
}

fn modify_string (mut s: String) -> String {
    s.push_str("あああ");
    s
}

// 単純ケース
enum Breakfast {
    Rice,
    Bread,
}

// 引数(?)を取れる Debug trait を利用
#[derive(Debug)]
enum Breakfast2 {
    Kind(String),
    Calorie(u32),
}

// すぐ↓で、自分で、std::fmt::Display を実装している
enum Breakfast3 {
    Kind(String),
    Calorie(u32),
}

// 下記からパクったけど赤くなるけど、特に文法エラーではない
// https://users.rust-lang.org/t/printing-enum-values/21002
impl std::fmt::Display for Breakfast3 {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Breakfast3::Kind(v) => v.fmt(f),
            Breakfast3::Calorie(v) => v.fmt(f),
        }
    }
}

// 自分で trait を定義する
pub trait Creature {
    // walk に デフォルトの定義を当てれる
    fn walk(&self) {
        println!("デフォルトで歩きます")
    }
    // こちらは関数の引数・戻り値の定義だけ
    fn weapon(&self) -> String;
}

// person モジュール
mod person {
    pub struct Person {
        pub name: String, // pub を付けないと private です
        height: u32,
        weight: u32,
        weapon: String,
    }

    impl Person {
        // コンストラクタだが、 new に特に意味はない
        pub fn new(name: String, height: u32, weight: u32, weapon: String) -> Person {
            Person {
                name,
                height,
                weight,
                weapon,
            }
        }
        pub fn height(&self) -> u32 {
            self.height
        }
        pub fn weight(&self) -> u32 {
            self.weight
        }
    }

    // Person に Creature trait を実装する
    impl super::Creature for Person {
        fn walk(&self) {
            println!("人として歩きます")
        }
        fn weapon(&self) -> String {
            self.weapon.clone()
        }
    }
}

// animal モジュール
mod animal {
    pub struct Animal {
        pub name: String, // pub を付けないと private です
        height: u32,
        weight: u32,
        weapon: String,
    }

    // コンストラクタだが、 new に特に意味はない
    impl Animal {
        pub fn new(name: String, height: u32, weight: u32, weapon:String) -> Animal {
            Animal {
                name,
                height,
                weight,
                weapon,
            }
        }
        pub fn height(&self) -> u32 {
            self.height
        }
        pub fn weight(&self) -> u32 {
            self.weight
        }
    }

    // Animal に Creature trait を実装する(walk はデフォルトにして、weapon() だけ実装)
    impl super::Creature for Animal {
        fn weapon(&self) -> String {
            self.weapon.clone()
        }
    }

}

// Tに特に意味はない(Uでもいい。何らかの型という意味)。ここ(T:Crature)では、Creature trait を実装している型のこと
fn what_weapon_has<T:Creature>(t :T) -> String {
    t.weapon()
}