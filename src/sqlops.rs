use rbatis::crud::CRUD;
use rbatis::crud_table;
use rbatis::impl_field_name_method;

#[crud_table]
#[derive(Clone, Debug)]
pub struct Posts {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    //pub comment: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
}

// this macro will create impl BizActivity{ pub fn id()->&str ..... }
impl_field_name_method!(Posts {
    id,
    title,
    content,
    //comment,
    create_time
});

/// (optional) manually implement instead of using `derive(CRUDTable)`. This allows manually rewriting `table_name()` function and supports  code completion in IDE.
/// (option) but this struct require  #[derive(Serialize,Deserialize)]
// use rbatis::crud::CRUDTable;
//impl CRUDTable for BizActivity {
//    fn table_name()->String{
//        "biz_activity".to_string()
//    }
//    fn table_columns()->String{
//        "id,name,delete_flag".to_string()
//    }
//}
//#[tokio::main]

pub async fn mytest_db_crud() -> Vec<Posts> {
    /// enable log crate to show sql logs
    //fast_log::init(fast_log::config::Config::new().console());
    /// initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
    let rb = rbatis::rbatis::Rbatis::new();
    println!("in mydb_test func1");
    /// connect to database  
    let linkstatus = rb.link("sqlite://../../sqlite3.db").await;
    match linkstatus {
        Ok(_) => println!("connect to database success"),
        Err(x) => println!("connect to database error {}", x),
    }
    println!("in mydb_test func");
    /// fetch allow None or one result. column you can use BizActivity::id() or "id"
    let result: Option<Posts> = rb.fetch_by_column(Posts::id(), 1).await.unwrap();
    //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ?
    println!("一条记录： {:?}", result);
    /*  query all */
    let result: Vec<Posts> = rb.fetch_list().await.unwrap();
    //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1
    println!("所有记录： {:?}", result);

    let post = Posts {
        id: Some(4),
        title: Some(String::from("title4")),
        content: Some(String::from("content4")),
        create_time: Some(rbatis::DateTimeNative::now()),
    };
    /// saving
    rb.save(&post, &[]).await;
    //Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

    ///query by id vec
    let result: Vec<Posts> = rb.fetch_list_by_column("id", &[2, 4]).await.unwrap();
    println!("指定id=2,4 的所有记录： {:?}", result);
    //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?)

    /*
    ///query by wrapper
    let result: Result<Option<Posts>, Error> =
        rb.fetch_by_wrapper(rb.new_wrapper().eq("id", 1)).await;
    println!("指定id=1 的所有记录： {:?}", result);
    //Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ?
    */

    ///delete
    rb.remove_by_column::<Posts, _>("id", 1).await;
    //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1

    let result: Vec<Posts> = rb.fetch_list().await.unwrap();
    //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1
    println!("删除id=1以后的所有记录： {:?}", result);

    /*
    ///delete batch
    rb.remove_batch_by_column::<Posts, _>("id", &["1", "2"])
        .await;
    //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  )
    */

    ///update
    //let activity = "posts";
    let mut post = post.clone();
    post.title = Some(String::from("new title 4"));
    let r = rb.update_by_column("id", &post).await;
    //Exec   ==> update biz_activity set  status = ?, create_time = ?, version = ?, delete_flag = ?  where id = ?
    /*rb.update_by_wrapper(
        &activity,
        rb.new_wrapper().eq("id", "12312"),
        &[Skip::Value(&serde_json::Value::Null), Skip::Column("id")],
    )
    .await;*/
    match r {
        Ok(res) => println!("update column to new title ok"),
        Err(e) => println!("update column to new title err {}", e),
    }
    //Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ?

    //let ret: i32 = 42;
    //future::ok(ret)
    return result;
}
