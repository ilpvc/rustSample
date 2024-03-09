#[cfg(test)]
mod tests{
    use crate::sql::sqlite;
    use crate::sql::sqlite::curd::select_all_person;

    #[test]
    fn test_sqlite(){
        let connection = sqlite::curd::create_table();
        match connection {
            Ok(_connect)=>{
                println!("创建成功！:")
            },
            Err(sqlite)=>{
                println!("创建失败！：{}",sqlite)
            }
        }
    }
    #[test]
    fn test_sqlite_insert(){
        let name = String::from("colzry");
        let age = 18;

        let person = sqlite::curd::Person{
            id:0,
            name,
            age,
        };
        sqlite::curd::insert_person(person)
    }

    #[test]
    fn test_select_person(){
        let result = select_all_person();
        match result {
            Ok(persons)=>{
                for person in persons {
                    println!("person name:{}",person.name)
                }
            },
            Err(sqlite)=>{
                println!("err:{}",sqlite)
            }
        }
    }
}