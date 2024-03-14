
#[cfg(test)]
mod tests{
    use std::vec;

    #[test]
    fn test_box(){
        let c = Box::new("ssss");
        println!("{}",c)
    }

    trait Draw {
        fn draw(&self);
    }

    struct Circle{
        r:f32,
    }
    struct Square{
        width:i32,
        height:i32
    }

    impl Draw for Circle {
        fn draw(&self) {
            println!("画一个圆,半径是{}",&self.r)
        }
    }

    impl Draw for Square {
        fn draw(&self) {
            println!("画一个方形，长：{}，宽：{}",&self.height,self.width)
        }
    }
    #[test]
    fn test_trait(){
        let shapes:Vec<Box<dyn Draw>> = vec![
            Box::new(Circle{r:32.0}),
            Box::new(Square{width:23,height:99})
        ];
        shapes.iter().for_each(|s|s.draw())
    }
}