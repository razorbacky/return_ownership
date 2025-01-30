[Rust] Return Ownership
===

상황은 다양하더라도 변수의 소유권 규칙은 언제나 동일하게 동작한다. 어떤 값을 다른 변수에 대입하면 값이 이동하고, heap에 데이터를 갖는 변수가 Scope를 벗어나면, 사전에 해당 데이터가 이동하여 소유권이 다른 변수에 이동되지 않는 이상 drop 에 의해 데이터가 제거된다.

이런 방식이 동작하더라도, 모든 함수가 소유권을 가졌다가 반납하는 것은 번거롭다. 함수에 넘겨줄 값을 함수 호출 이후에도 사용하고 싶은데, 그렇다고 함수로부터 얻고자 하는 결과에 더해서 이후 다시 쓰고 싶은 변수까지 같이 반환 받아야 한다면, 본말전도나 다름없다.

그럼, 함수가 값을 사용할 수 있도록 하되 소유권은 가져가지 않도록 하고 싶다면 어떻게 할까?

```rust
// Argument Ownership 25.01.30
// 매개변수의 소유권을 되돌려주는 방법

/* 함수에 넘겨줄 값을 함수 호출 이후에도 쓰고 싶고, 다시 쓰고 싶은 변수까지 같이 반환해야 한다면 복잡해진다.
함수가 값을 사용할 수 있도록 하되, 소유권은 가져가지 않도록 하고 싶다면. */

fn main() {
    let s1 = String::from("Hello"); // s1에 'Hello' String을 저장한다.

    let (s2, len) = calculate_length(s1);
    // calculate_length(s1)에 s1의 인자를 전달한다. (s1 인자 = hello)
    // calculate_length 함수의 결괏값에서 length 값을 가져온다.

    println!("The length of '{s2}' is {len}");
    // s2('hello')와 len('5') 을 출력한다.
    // 결과 출력 : "The length of 'Hello' is 5"
}

fn calculate_length(s: String) -> (String, usize) {
    let length  = s.len(); // len()은 String의 길이를 반환한다.
    // 인자로부터 전달받은 'Hello' 는 5글자로 string 값은 "Hello", length는 '5글자' 를 반환한다.

    (s, length)
}
```

