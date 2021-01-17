

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

    println!("{:?}", 1.0 < 1.1);
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