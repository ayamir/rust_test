use stack_test::Stack;

fn main() {
    #[derive(PartialEq, Eq, Debug)]
    struct TestStruct {
        a: i32,
    }

    let a = TestStruct { a: 5 };
    let b = TestStruct { a: 9 };

    let mut s = Stack::<&TestStruct>::new();
    assert_eq!(s.pop(), None);

    s.push(&a);
    s.push(&b);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&b));
    assert_eq!(s.pop(), Some(&a));
    assert_eq!(s.pop(), None);
}
