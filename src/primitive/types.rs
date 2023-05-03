fn main() {
    println!("primitive/types!!!");

    println!(
        "\n| {} |-----------------------------------------------------\n",
        "result type"
    );

    let ok_result: Result<i32, String> = Ok(1);
    let error_result: Result<i32, String> = Err("Error message".to_string());
    println!("{}", ok_result.unwrap());
    println!("{}", error_result.unwrap_or(200));

    fn sample_func(result: i32) -> Result<i32, String> {
        println!("sample-func / {:?}", result);
        Ok(200)
    }

    println!("\n-----------------------------------------------------");

    println!(
        "\n| {} |-----------------------------------------------------\n",
        "and_then"
    );
    let other_result: Result<i32, String> = Ok(200);
    let _next_result = other_result.and_then(sample_func);
    println!("\n-----------------------------------------------------");
}
