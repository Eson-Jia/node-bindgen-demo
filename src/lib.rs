use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn sum(first: i32, second: i32) -> i32 {
    first + second
}

#[node_bindgen(name = "sum_optional")]
//发现一个问题 sum_optional 不设置名字(name=)调用时就报错函数不存在
//将 _ 去掉就不会有该问题,神奇的 bug
fn sum_optional(first: i32, second: Option<i32>) -> i32 {
    first + second.unwrap_or(0)
}

#[node_bindgen(name = "multiply")]
fn mul(first: i32, second: i32) -> i32 {
    first * second
}


#[node_bindgen]
fn hello<F: Fn(String)>(first: f64, second: F) {
    let msg = format!("argument is: {}", first);
    second(msg);
}

// #[node_bindgen]
// async fn hello_async(arg: f64) -> f64 {
//     println!("sleeping");
//     sleep(Duration::from_secs(1)).await;
//     println!("woke and adding 10.0");
//     arg + 10.0
// }