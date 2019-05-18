pub mod jieba;

// pub fn cuttest() {
//     let test_str = "我爱北京天安门";
//     let words = jieba::cut(test_str.to_string());
//     for word in &words {
//         print!("{}", word);
//     }
// }

fn test() {
    for i in 0..3 {
        let mut a:Vec<usize> = Vec::new();
        if a.is_empty() {
            a.push(3);
        }
        a.push(i);
    }
}