extern crate jieba;

// use jieba;


#[test]
fn it_works() {
    let result = jieba::jieba::cut(String::from("我来到北京清华大学"));
    // assert_eq!(vec!("我", "来到", "北京", "清华", "清华大学", "华大", "大学"), result);
    assert_eq!(vec!("我", "来到", "北京", "清华大学"), result);
    // assert_eq!(vec!(String::from("他"), String::from("hdk")), result);

    // jieba::cuttest();
}


// #[test]
// fn test_2() {
//     assert_eq!(2 + 2, 4); / / / / 

// }

