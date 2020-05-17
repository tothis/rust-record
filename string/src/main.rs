fn main() {
    // 默认声明变量 为不可变变量
    let str = String::from("0123456789");
    let s1 = &str[0..5];
    let s2 = &str[5..10];
    let s3 = &str[0..=4];
    let s4 = &str[5..=9];
    println!("s1 -> {}, s2 -> {}, s3 -> {}, s4 -> {}", s1, s2, s3, s4);

    // mut(mutability)代表可变变量
    let mut mut_str = String::from("abcd,efgh");
    let offset = mut_str.find(',').unwrap_or(mut_str.len());
    // 获取`,`前的字符串
    let new_mus_str: String = mut_str.drain(..offset).collect();
    println!("old:{}, new:{}", mut_str, new_mus_str);

    // 断言
    assert_eq!(new_mus_str, "abcd");
    assert_eq!(mut_str, ",efgh");

    // 清空字符串
    mut_str.drain(..);
    assert_eq!(mut_str, "");

    let mut mut_str = String::from("abcd,efgh");
    let beta_offset = mut_str.find(',').unwrap_or(mut_str.len());
    // Replace the range up until the β from the string
    mut_str.replace_range(..beta_offset, "1234");
    println!("old:{}", mut_str);
    assert_eq!(mut_str, "1234,efgh");

    // 获取字符串字节长度
    let len = "test".len();
    assert_eq!(4, len);
    let len = "测试".len();
    assert_eq!(6, len);

    println!("{}", "12321".trim_matches('1'));
    println!("{}", "12321".trim_matches(char::is_numeric));
    let x: &[_] = &['1', '2'];
    println!("{}", "12321".trim_matches(x));

    // 返回一个迭代器 该迭代器使用char::escape_debug转义每个char
    for c in "0\n!".escape_debug() {
        println!("{}", c);
    }
    println!("{}", "0\n!".escape_debug());

    println!("0\n!");
    println!("0\\n!");
    for c in "abcd".escape_debug() {
        println!("{}", c);
    }
}