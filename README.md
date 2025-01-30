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
    

    (s, length) // 인자로부터 전달받은 'Hello' 는 5글자로 string 값은 "Hello", length는 '5글자' 를 반환한다.
}
```

**결과 출력**

```
The length of 'Hello'  is 5
```

### 배울 점
기본적으로 Rust는 소유권이 전달되면, 반환이 되기 전까지 소유권을 사용할 수 없다.
이 코드에서 s1은 calculate_length 함수로 전달되었기 때문에, s1 변수를 재사용할 수 없다.
그러나, ***'.clone'*** 을 통해 deep copy로 재사용 할 수 있다.

```rust
fn main() {
    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1.clone()); // s1을 복제하여 사용하기 위해서는 모두 .clone 을 적용해야 한다.

    println!("The length of '{s2}' is {len}");

    println!("{}", s1.clone());
    // ".clone" 을 이용한 Deep Copy 를 통해 변수 그 자체를 복제하여 이용할 수 있다. 단, 동일한 변수를 사용한 모든 곳에 .clone 을 해줘야 한다. 10번 라인 참조.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
```

위와 같이 s1 변수가 필요한 부분에 ***'s1.clone'*** 을 적용하여 calculate_length의 매개변수로 s1을 전달하고, 후에 println!에 s1.clone 으로 중복하여 사용할 수 있다.

이를 통해, calculate_length에도 s1 변수의 값인 "Hello" 를 전달하고, 56번 라인의 println!("{}"m s1.clone())l 에도 "Hello" 를 전달해서 중복하여 사용한다.

**결과 출력**

```
The length of 'Hello' is 5
Hello
```

