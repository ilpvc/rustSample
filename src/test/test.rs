#[cfg(test)]
mod tests{
    use crate::sql::sqlite;
    use crate::sql::sqlite::curd::{delete_person_by_id, Person, select_all_person, select_person_by_id, update_person_by_id};

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

    #[test]
    fn test_query_by_id(){
        let person = select_person_by_id(1);
        println!("id:{},name:{},age:{}",person.id,person.name,person.age)
    }

    #[test]
    fn test_update_by_id(){
        let person = Person{
            id:1,
            name: String::from("ilpvc的老大哥"),
            age: 25
        };
        let result = update_person_by_id(person).expect("error");
        println!("{}",result)
    }

    #[test]
    fn test_delete_by_id(){
        let id = 2;
        let i = delete_person_by_id(2);
        println!("{}",i)
    }
}