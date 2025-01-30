// Argument Ownership 25.01.30
// 매개변수의 소유권을 되돌려주는 방법

/* 함수에 넘겨줄 값을 함수 호출 이후에도 쓰고 싶고, 다시 쓰고 싶은 변수까지 같이 반환해야 한다면 복잡해진다.
함수가 값을 사용할 수 있도록 하되, 소유권은 가져가지 않도록 하고 싶다면. */

fn main() {
    let s1 = String::from("Hello"); // s1에 'Hello' String을 저장한다.

    let (s2, len) = calculate_length(s1.clone());
    // calculate_length(s1)에 s1의 인자를 전달한다. (s1 인자 = hello)
    // calculate_length 함수의 결괏값에서 length 값을 가져온다.

    println!("The length of '{s2}' is {len}");
    // s2('hello')와 len('5') 을 출력한다.
    // 결과 출력 : "The length of 'Hello' is 5"

    println!("{}", s1.clone());
    // ".clone" 을 이용한 Deep Copy 를 통해 변수 그 자체를 복제하여 이용할 수 있다. 단, 동일한 변수를 사용한 모든 곳에 .clone 을 해줘야 한다. 10번 라인 참조.
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    // len()은 String의 길이를 반환한다.
    // 인자로부터 전달받은 'Hello' 는 5글자로 string 값은 "Hello", length는 '5글자' 를 반환한다.

    (s, length)
}
