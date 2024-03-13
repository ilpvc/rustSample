#[cfg(test)]
mod tests {
    use crate::sql::mysql::curd::{create_dept_table, delete_dept_by_id, insert_dept, select_all_dept, select_by_id, update_dept_by_id};
    use crate::sql::mysql::dept::Dept;
    use crate::sql::sqlite;
    use crate::sql::sqlite::curd::{delete_person_by_id, Person, select_all_person, select_person_by_id, update_person_by_id};

    #[test]
    fn test_sqlite() {
        let connection = sqlite::curd::create_table();
        match connection {
            Ok(_connect) => {
                println!("创建成功！:")
            }
            Err(sqlite) => {
                println!("创建失败！：{}", sqlite)
            }
        }
    }

    #[test]
    fn test_sqlite_insert() {
        let name = String::from("colzry");
        let age = 18;

        let person = Person {
            id: 0,
            name,
            age,
        };
        sqlite::curd::insert_person(person)
    }

    #[test]
    fn test_select_person() {
        let result = select_all_person();
        match result {
            Ok(persons) => {
                for person in persons {
                    println!("person name:{}", person.name)
                }
            }
            Err(sqlite) => {
                println!("err:{}", sqlite)
            }
        }
    }

    #[test]
    fn test_query_by_id() {
        let person = select_person_by_id(1);
        println!("id:{},name:{},age:{}", person.id, person.name, person.age)
    }

    #[test]
    fn test_update_by_id() {
        let person = Person {
            id: 1,
            name: String::from("ilpvc的老大哥"),
            age: 25,
        };
        let result = update_person_by_id(person).expect("error");
        println!("{}", result)
    }

    #[test]
    fn test_delete_by_id() {
        let i = delete_person_by_id(2);
        println!("{}", i)
    }

    #[test]
    fn test_mysql_create_table() {
        create_dept_table()
    }

    #[test]
    fn test_mysql_insert() {
        let dept = Dept {
            id: 0,
            dept_name: String::from("安全部"),
            created_time: String::from("2024-04-22 12:10:45"),
        };
        insert_dept(dept)
    }

    #[test]
    fn test_mysql_select_by_id() {
        let option = select_by_id(1);
        let dept = option.unwrap();
        println!("{},{},{}", dept.id, dept.dept_name, dept.created_time);
    }

    #[test]
    fn text_mysql_all() {
        let dept = Dept {
            id: 0,
            dept_name: String::from("colzry"),
            created_time: String::from("192.168.190,11"),
        };

        let new_dept = Dept{
            id:1,
            dept_name: String::from("new ilpvc"),
            created_time: String::from("2014-19-19 sjdksjdkas")
        };
        insert_dept(dept);
        select_all_dept().iter().for_each(|i|println!("{},{},{}",i.id,i.dept_name,i.created_time));
        update_dept_by_id(new_dept);
        select_all_dept().iter().for_each(|i|println!("{},{},{}",i.id,i.dept_name,i.created_time));
        delete_dept_by_id(1);
        select_all_dept().iter().for_each(|i|println!("{},{},{}",i.id,i.dept_name,i.created_time));
    }
}