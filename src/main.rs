use jsonparser;

fn main() {
    let x = jsonparser::parse(
        r#"[
        4.5, null, "hello world", true, { "key1": false },
        [ false, true, "hello", null],
        {
            "key1": true,
            "key2": false,
            "key3": [0, 1, 2, 3]
        }
    ]"#,
    )
    .unwrap();
    println!("{}", x);
}
