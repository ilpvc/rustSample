use mysql::*;
use mysql::prelude::Queryable;
use crate::sql::mysql::dept::Dept;
use crate::sql::sqlite::curd::Person;

fn create_connect() -> PooledConn {
    let url = "mysql://root:@8.137.xxx.xxx:3306/rust_sample";
    let pool = Pool::new(url).expect("连接数据库失败");
    let connect = pool.get_conn().expect("error");
    connect
}

pub fn create_dept_table() {
    let mut conn = create_connect();
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS dept (
            id INTEGER PRIMARY KEY,
            dept_name Varchar(255) NOT NULL,
            created_time Varchar(255) NOT NULL
        )").expect("创建失败");
}

pub fn insert_dept(dept: Dept) {
    let mut conn = create_connect();
    conn.exec_drop(
        r"INSERT INTO dept (dept_name,created_time) values (?,?)",
        (dept.dept_name, dept.created_time),
    ).expect("插入数据失败")
}

pub fn select_by_id(ids: i32) -> Option<Dept> {
    let mut coon = create_connect();

    let result = coon.exec_map(
        "select * from dept where id=?",
        (ids, ),
        |(id, dept_name, created_time)| {
            Dept { id, dept_name, created_time }
        },
    ).expect("查询失败");
    if let Some(dept) = result.into_iter().next() {
        Some(dept)
    } else {
        None
    }
}