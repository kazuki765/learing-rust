// fn main() {
//     let s = "hello";
//     println!("{s}");

//     let mut s = String::from(s);
//     s.push_str(", world!");
//     println!("{s}");

//     let s1 = String::from(s);
//     let s2 = s1.clone();
//     println!("{s1}");
// }

// // move and copy
// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here
//     println!("{s}")                 // This is invalid.

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward

// fn main() {
//     let mut s1 = String::from("hello");

//     // &をつけることで、s1を参照するが、所有しない
//     // つまりこの先でs1はそのまま使い続けられる。
//     let len = calculate_length(&s1);

//     // let s2 = s1;
//     // let s3 = s1;

//     change(&mut s1);

//     println!("s1: {}, len: {}", s1, len);
// }

// // この関数ではsを所有しないと定義されているので、関数終わりにdropされない。
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// // 借用しているものはデフォルトだとへんこうできないが、mutキーワードを使ってmutableにすることもできる
// // その場合、変数定義時にmutキーワードを使い、関数利用時にもmutキーワードを使う必要がある
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");

//     // 普遍借用は複数OK
//     let r1 = &s;
//     let r2 = &s;
//     let r3 = &mut s; //これはNG。

//     println!("{}, {}, and {}", r1, r2, r3);
// }

// fn main() {
//     let mut s = String::from("hello");

//     // この定義方法はOK
//     // 理由は、r1,r2(不変借用)とr3(可変借用)の利用スコープが明確に分かれているため。
//     // Rustコンパイラはあるスコープ内で参照がどこまで利用されているか？を知ることができる。
//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);

//     let r3 = &mut s;

//     println!("{r3}");
// }

// // Dangling References
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello"); // ここでsというStringインスタンスが作成される

//     &s // 本来ここでsはdropされるが、参照がReturnされている。
//        // Rustはこれを許さない。（このままでは、dropされた何もない参照をかえしてしまう。）
//        // これを防ぐにはStringを直接返す（借用ではなく、所有権を移す）必要がある
// }

// 4-3 The Slice Type(ASCIIのみ)
// fn main() {
//     let s = String::from("hello world");
//     // let location = first_word(&s);

//     // Slice. sの参照するメモリ内の一部分を参照できる。
//     // つまり、sのライフサイクルとhello , worldのライフサイクルは同じ。
//     let hello = &s[0..5];
//     // let hello = &s[..5]; // 同じ
//     let world = &s[6..11];
//     // len world = &s[6..]; // 同じ
//     let zenbu = &s[0..11];
//     let zenbu = &s[..]; // 同じ

//     println!("{}, {}, {}", hello, world, zenbu);
// }

// // Sringを１文字ずつ調べ、スペースかどうかチェックし、スペースのindexを返す。
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         // byteリテラル。スペースのBit列と比較してる
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn main() {
//     // let riteral = "sss"; // こやつはSlice（stringの不変の参照）
//     let s = String::from("hello world");

//     let first = first_word(&s);

//     // Sliceでライフサイクルを統一することで、不正なタイミングでメモリから削除しようとした時にコンパイルエラーにしてくれる。
//     // s.clear();

//     println!("{first}");
// }
// // Sliceを返す場合は＆str
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

// Slice array
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[..3];
    assert_eq!(slice, &[2, 3]);
}
