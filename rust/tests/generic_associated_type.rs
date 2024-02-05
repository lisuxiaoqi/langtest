/*
   关联类型（associated_type)，是指在trait中把泛型定义为内部类型的方式：
   trait T{
        type A;
        type B;
   }
   而不是常规的：
   trait I<A, B>{}

   关联类型的好处，可能在于书写更美观简洁。关联类型仅能用在trait中，不能用于struct。
*/

//例子一，使用关联类型
#[test]
fn test_associated_type_1() {
    //定义一个Iterator
    //可以返回容器中下一个Item
    //其中Item是一个泛型，使用关联类型方式表示
    trait Iterator {
        type Item;
        fn next(&mut self) -> Option<&Self::Item>;
    };

    //定义一个实现，实现Iterator
    //在实现中，指定Iterator的关联类型
    struct ArrayIterator<T> {
        array: Vec<T>,
        index: usize,
    };

    impl<T> Iterator for ArrayIterator<T> {
        type Item = T;

        fn next(&mut self) -> Option<&Self::Item> {
            if self.index < self.array.len() {
                let item = Some(&self.array[self.index]);
                self.index += 1;
                item
            } else {
                None
            }
        }
    }

    //使用具体的类型
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = ArrayIterator { array: v, index: 0 };
    while let Some(item) = iter.next() {
        println!("Iterator item:{}", item);
    }
}

//例子二，使用关联类型
#[test]
fn test_associated_type_2() {
    //定义一个trait, 使用关联类型
    trait Contains {
        type A;
        type B;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }

    //实现trait
    struct Container(i32, i32);
    impl Contains for Container {
        type A = i32;
        type B = i32;

        fn first(&self) -> i32 {
            self.0
        }

        fn last(&self) -> i32 {
            self.1
        }
    }

    //trait的实现作为参数传递
    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }

    //调用函数
    let ref c = Container(10, 20);
    let delta = difference(c);
    println!("Call difference:{}", delta);
}
