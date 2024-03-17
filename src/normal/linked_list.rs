pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/*
 *  使用枚举创造链表，Box设置只能指针，len和stringify使用递归
 */
impl List {
    pub fn new() -> List {
        List::Nil
    }

    pub fn append(self, elem: i32) -> List {
        List::Cons(elem, Box::new(self))
    }

    pub fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0
        }
    }

    pub fn stringify(&self)->String{
        match *self {
            List::Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil")
            },
        }
    }
}