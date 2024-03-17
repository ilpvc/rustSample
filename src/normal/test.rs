#[cfg(test)]
mod tests{
    use crate::normal::linked_list::List;

    #[test]
    fn test1(){
        let list = List::new();
        let list = list.append(1);
        let list =list.append(2);
        let list =list.append(3);
        println!("{}", list.len());
        println!("{}",list.stringify())

    }
}