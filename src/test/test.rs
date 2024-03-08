#[cfg(test)]
mod tests{
    use crate::sql::sqlite;
    #[test]
    fn test_sqlite(){
        let connection = sqlite::curd::create_db();
        match connection {
            Ok(_connect)=>{
                println!("创建成功！:")
            },
            Err(sqlite)=>{
                println!("创建失败！：{}",sqlite)
            }
        }
    }
}