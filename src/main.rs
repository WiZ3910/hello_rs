use core::future::Future;



fn main() {
    let s1: String = String::from("Hello world!");
    let s2: &str = &s1;
    let s3:String = s2.to_string(); //&str --> string
    
    let mut t = (1, "2");

    t.0 = 2;
    t.1 = "3";

    let mut a: [i32; 3] = [0,1,2];
    let b: [i32;3] = [0;3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[0..3]);

    let p = Person {
        name : String::from("Josh"),
        age: 8,
    };

    let literal = "Hello";

    enum  Event {
        Quit,
        KeyDown(u8),
        MouseDown {x:i32, y:i32},
    }
    //代数データ型を使うことが出来る！！嬉しい！！
    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x:10, y:10};

    let result : Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code : {}", code),
        Err(err)
         => println!("Err : {}", err)
    };

    let result: Result<i32, String> = Ok(200);
    if let Ok(code) = result {
        println!("code : {}", code);
    }

    println!("code : {}", result.unwrap_or(-1));

    let result : Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));

    fn func(code : i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }

    let result : Result<i32, String> = Ok(100);
    let next_result = result.and_then(func);
    let result : Result<i32, String> = Err("error".to_string()); // func() は実行されない

    fn error_handling(result : Result<i32, String>) -> Result<i32,String> {
        let code = result?;
        println!("code: {}",code);
        Ok(100)
    }
    
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![0;5];
    let v = vec![1,2,3,4,5];
    //println!("{}", v[0]);
    for element in v {
        println!("{}", element);
    }

    let byte_array = [b'h',b'e',b'l',b'l',b'o'];
    print(Box::new(byte_array));

    fn print(s : Box<[u8]>) {
        println!("{:?}",s);
    }

    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("number == 0");
    }

    let number = 1;
    let result = if 0 <= number { number } else {-number};

    let mut count = 0;
    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };

    let mut count = 0;
    while count < 10 {
        println!("count : {}", count);
        count += 1;
    }

    let count : i32;
    for count in 0..10 {
        println!("count : {}" , count);
    }

    let array = [0,1,2,3,4,5,6,7,8,9];

    for element in &array {
        println!("element: {}", element);
    }

    // 'main : loop {
    //     println!("main loop start");
    //     'sub : loop {
    //         println!("sub loop start");
    //         break 'main;
    //         println!("sub loop end"); //表示されない
    //     }
    //     println!("main loop end"); // 表示されない
    // }

    let i : i32 = 1;
    match i  {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"), //アンダースコアはあらゆる値にマッチする。
    }

    enum Color {
        Red,Blue,Green,
    }
    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        _ => println!("misc")
    }

    let result : Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number ) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };

    for number in i .. 5 {
        println!("{}", number);
    }

    struct Iter {
        current : usize,
        max: usize,
    };

    impl Iterator for Iter {
        type Item = usize; //出力する型の紐付け
        fn next(&mut self) -> std::option::Option<usize> {
            self.current += 1;
            if self.current - 1 < self.max {
                Some(self.current - 1)
            } else {
                None
            }
        }
    };

    let it = Iter {
        current : 0,
        max: 10
    };

    for num in it {
        println!("num: {}", num);
    }

    let x = add(1,2);
    println!("x = {}", x);

    fn add(a : i32, b : i32) -> i32 {
        a + b
    }

    let p = Person {
        name : "Taro".to_string(),
        age: 20,
    };
    // p.say_name();
    // p.say_age();
    p.say_name_and().say_age();
    p.say_age_and().say_name();
    
    let s = "hello";
    let s = format!("{}-{:?}", s, ("D", 5));
    println!("{}",s);
    let s = format!("{}{}", "abc", "def");
    println!("{}",s);
    let s = concat!("A", "b2", 3);
    println!("{}",s);

    println!("{:?}",[1,2,3,4,5]);
    //println!("{}", [1,2,3,4,5]);

    use std::io::Write;
    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    write!(&mut w, "is 123");
    dbg!(w);

    //panic!("intended panic");

    let v = vec![1,2,3];
    
    println!("defined in file: {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    assert!(true);
    assert_eq!(1,1);
    assert_ne!(1,0);
    //debug_assert!(false);
    debug_assert_eq!(1,1);
    debug_assert_ne!(1,0);
    //assert!(false);
    println!("assertion has passed");


    struct HappyPerson {
        name : String,
        state : Emotion,
    }
    
    impl Emotional for HappyPerson {
        fn get_anger(&mut self) -> std::string::String { unimplemented!() /*わけがあって、今は実装していないが、コンパイルの検査を通過する*/}
        fn get_happy(&mut self) -> std::string::String { format!("{} is always happy.", self.name) }
        fn tell_state(&mut self) -> std::string::String { todo!()  /*後で実装したいが、とりあえずコンパイルの検査を通す*/}
    }

    //unreachableの使用例
    fn f(x : usize) -> &'static str {
        match x {
            n if n * n % 3 == 0 => "3n",
            n if n * n % 3 == 1 => "3n+1 or 3n+2",
            _ => unreachable!(),//コンパイラは、上記条件で網羅していることを判定できない
        }
    }

    //ビルトインマクロ一覧
    //cfg!
    //file!
    //concat!
    //env!
    /*
    主にプログラム外のリソースにアクセスするなど、プログラム内の関数からは実現できない処理を実現するために、コンパイラ側で実装されている。
    */

    //標準トレイトの導出
    #[derive(Eq,PartialEq)]
    struct A(i32);
    #[derive(PartialEq, PartialOrd)]
    struct B(f32);
    #[derive(Copy,Clone)]
    struct C(i32);
    #[derive(Clone)]
    struct D(i32);
    #[derive(Debug)]
    struct E;
    #[derive(Default)]
    struct F;

    println!("{:?}", A(0) == A(1));
    println!("{:?}", B(1.0) > B(0.0));
    let c0 = C;
    let _c1 = c0;
    let _c2 = c0; // Cがムーブならc0は_c1にムーブしているのでここでコンパイルエラー

    //DはClone可能
    let d0 = D;
    let _d1 = d0.clone();

    println!("{:?}", E);

    //Fはdefault可能
    let _f = F::default();

    //PartialEq, PartialOrdについて
    /*f32は、NaNという無効な値が存在するため、
    反射率 : a == a
    対象率 : b == a => a == b
    推移率 : a == b && b == c => a == c
    を満たさない（全てfalse)
    
    このような値でも比較概念を与える為
    PartialEq, PartialOrd
    が存在する
    */
    let mut x = vec![0.1, 0.5, 0.3, 0.4, 0.2, /* 0.0/0.0 */];
    //NaNが存在すると、Unwrap時にpanicが起こる
    x.sort_by(|a,b| a.partial_cmp(b).unwrap());
    println!("{:?}", x);// [0.1, 0.2, 0.3, 0.4, 0.2]
    //partialさえ実装していれば、普通に、演算子を使うことも出来るようになる。
    println!("{:?}", 1.0 < 1.1);

    /*ゼロコスト抽象化*/
    trait Tweet {
        fn tweet(&self);
        //デフォルト実装も追加することが出来るのか。
        fn tweet_twice(&self) {
            self.tweet();
            self.tweet();
        }
        fn shout(&self) {
            //これは笑った
            println!("Uooooooooooohhh!!!");
        }
    }
    struct Dove; //鳩
    struct Duck; //アヒル
    impl Tweet for Dove {
        fn tweet(&self) { println!("Coo!"); }
    }
    impl Tweet for Duck { 
        fn tweet(&self) { println!("Quack!");}
    }

    let dove = Dove;
    dove.tweet();
    dove.tweet_twice();
    dove.shout();
    let duck = Duck;
    //動的ディスパッチ
    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in bird_vec {
        bird.tweet();
    }
    //マーカートレイトについて
    /*
    - Copy : 値の所有権を渡す代わりに、値のコピーを行うようにする(所有権は後ほど)
    - Send : スレッド境界を超えて、所有権を転送できることを示す
    - Sized: メモリ上でサイズが決まっていることを示す
    - Sync : スレッド間で安全に参照を共有できることを示す
    */

    //ジェネリクス
    fn make_tuple<T,S>(t: T, s:S) -> (T,S) {
        (t,s)
    }
    let tp = ("I'm", Emotion::Happy);

    let mut important_data = "Hello World!".to_string();
    important_data = calc_data(important_data); //値の所有権を渡し、返してもらう。
    calc_data2(&important_data); // まあ、参照渡しだね。これを借用というらしい。



    fn calc_data(data: String) -> String {
        println!("{}",data);
        data
    }
    fn calc_data2(data: &String)  {
        println!("{}",data);
    }
{
    let x = 5;
    let y = &x; //１回目の不変な参照渡し
    let z = &x; //2回目の``
    dbg!(x);
    dbg!(y);
    dbg!(z);
}
{
    let mut x = 5;
    let y = &mut x;
    //let z = &mut x; //２回目の可変な参照渡し（エラー）
    dbg!(y);
    //dbg!(z);
}
{
    let mut x = 5;
    let y = &x;
    //let z = &mut x; //可変な参照渡し（これもエラー）
    dbg!(y);
    //dbg!(z);
}
    let y : i32 = i32::default();
    {
        let x = 5;
        //y = &x; // <- x does not live long enough.
        dbg!(x);
        //xのライフタイム終了
    }
    dbg!(y);
    //xよりyが長生きすることはできない。

    //スレッドと所有権の移動
    use std::thread;
    let mut handles = Vec::new();
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            println!("workder thread. : {}",i);
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
    
    use std::sync::{Arc,Mutex};
    //スレッド間の情報共有
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));
    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(std::thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1;
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
    dbg!(data);
    
    //メッセージパッシング方式で実装してみる
    use std::sync::mpsc;
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        //mainから各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        //各スレッドからmainへのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();
        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }
    //各スレッドにdataの値を送信
    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }
    //各スレッドからの結果をdataに格納
    for x in 0 ..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }
    for handle in handles { 
        let _ = handle.join();
    }
    dbg!(data);

    use futures:: {executor, future::join_all};
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    struct CountDown(u32);
    impl Future for CountDown {
        type Output = String;
        fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<String> {
            if self.0 == 0 {
                Poll::Ready("Zero!!".to_string())
            } else {
                println!("{}", self.0);
                self.0 -= 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    for(i, s) in res.iter().enumerate() {
        println!("{}:{}", i, s);
    }
    async fn async_add(left:i32, right:i32) -> i32 {
        left + right
    }
    async fn something_great_async_function() -> i32 {
        let ans = async_add(2,3).await; //この時点で５というあたいを取り出せる。
        //何か処理を挟むことも出来る。
        println!("{}",ans);
        ans
    }
    fn some_great_function() -> impl Future<Output = i32> {
        //let value : i32 = 5;
        async {
            let value : i32 = 5;
            send_to_another_thread_with_borrowing(&value).await
        }
        
    }

    async fn send_to_another_thread_with_borrowing(x : &i32) -> i32 {
        //何か別スレッドへ送る処理が書かれている想定
        4
    }
    executor::block_on(something_great_async_function());

    //tokioはRustをずっと支えてきた非同期ライブラリらしい。
    //今も多くのライブラリがtokioに依存している。
    //macrosフィーチャーを追加し、#[tokio::main]というアトリビュートを利用可能に。
    //これにより、main関数をasync化できる
    /*
    async fn add(left : i32, right: i32) -> i32 {
        left + right
    }
    #[tokio::main]
    async fn main() {
        let ans = add(2,3).await;
        println!("{}",ans)
    }
    */

    //async-std は近年登場してきたクレート。tokioは内部実装やAPIがやや複雑なので、それに対処するために
    //作られたが、まだ歴史は浅い。
    //attributes フィーチャーを追加することにより、tokioと同じように、async main化出来る。
    /*
    #[async_std::main]
    async fn main () {
        let ans = async_add(2,3).await;
        println!("{}", ans);
    }
    */

    /* async-trait */
    // Rustでは、現状トレイトの関数にasyncをつけることはできない。
    /*
    trait AsyncTrait {
        async fn f() {
            println!("Coundn't compile"); // コンパイルエラー
        }
    }
    */
    //トレイトに#[async_trait]というアトリビュートを追加すると、async fn ... という関数宣言を出来るようになる。
    use async_trait::async_trait;
    #[async_trait]
    trait AsyncTrait {
        async fn f() {
            println!("Counld compile");
        }
    }

    //実装を委ねることも出来るよ
    #[async_trait]
    pub trait AsyncTrait2 {
        async fn f(&self);
    }
    struct Runner{}
    #[async_trait]
    impl AsyncTrait2 for Runner {
        async fn f(&self) {
            println!("Hello, async-trait");
        }
    }
    //次は、クレートとモジュールの勉強に入るので、プロジェクトを切り替える。ここまで。
}

enum Emotion {
    Anger,
    Happy
}

trait Emotional {
    fn get_happy(&mut self) -> String;
    fn get_anger(&mut self) -> String;
    fn tell_state(&mut self) -> String;
}

struct Person {
    name : String,
    age : u32,
}

impl Person {
    fn say_name(&self) {
        println!("I am {}.", self.name);
    }

    fn say_age(&self) {
        println!("I am {} years old", self.age);
    }

    fn say_name_and(&self) -> &Self {
        println!("I am {}.", self.name);
        self
    }
    fn say_age_and(&self) -> &Self {
        println!("I am {} years old.", self.age);
        self
    }

}