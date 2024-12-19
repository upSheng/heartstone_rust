

#[cfg(test)]
mod tests {

    // 引入父模块中的内容
    use regex::Regex;


    #[test]
    fn test_add() {
        // 定义一个正则表达式
        let re = Regex::new(r"\d+").unwrap();

        // 要匹配的文本
        let text = "There area 42 apples and 13 oranges.";

        // 查找所有匹配项
        for mat in re.find_iter(text) {
            println!("Found match: {}", mat.as_str());
        }
    }
}