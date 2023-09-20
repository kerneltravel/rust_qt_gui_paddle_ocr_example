#![windows_subsystem = "console"] //要显示控制台时，仅启用这句
                                  //#![windows_subsystem = "windows"]  // 要隐藏时，仅启用这句

use cpp_core::{CastInto, NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, slot, QDateTime, QObject, QString, SlotNoArgs};
use qt_widgets::{QApplication, QFileDialog, QMessageBox, QWidget};
use serde_json;
use std::{rc::Rc, str::FromStr};
use std::future::Future;
use std::thread;
use serde::{Deserialize, Serialize};


//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;

use rbatis::crud::CRUD;
use rbatis::crud_table;
use rbatis::impl_field_name_method;


mod filemd5;
mod ocr;
mod pdf;
mod uic;

pub type OcrResult = Vec<Root>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "box")]
    pub box_points: Vec<BoxPoint>,
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "text")]
    pub text: String,
    /*#[serde(rename = "cls_label")]
    pub cls_label: i64,
    #[serde(rename = "cls_score")]
    pub cls_score: f64,*/
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
pub struct BoxPoint {
    #[serde(rename = "x")]
    pub x: i64,
    #[serde(rename = "y")]
    pub y: i64,
}

pub struct HelloWorld {
    ui: uic::MainWindow,
    //signal_quit: QBox<SignalOfInt>,
}

impl StaticUpcast<QObject> for HelloWorld {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.ui.widget.as_ptr().static_upcast()
    }
}

impl HelloWorld {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P) -> Rc<Self> {
        let this = Rc::new(Self {
            ui: uic::MainWindow::load(parent),
            //signal_quit: unsafe { Signal::new(q_object, receiver_id) },
        });
        unsafe { this.init() };
        this
    }

    unsafe fn init(self: &Rc<Self>) {
        /* add slot + signal connectors, etc. */
        self.ui
            .widget
            .set_window_title(&qs("OCR图片文字识别(示例，基于PaddleOCR + Rust QT)"));
        self.ui.btn_modify_bingli.set_text(&qs("选择图片"));
        self.ui
            .btn_modify_bingli
            .clicked()
            .connect(&self.slot_on_add_clicked());
    }

    unsafe fn show(self: &Rc<Self>) {
        self.ui.widget.show();
    }

    /// 将OCR识别到的json结果中的text 字段，合并成一个字符串
    unsafe fn get_ocr_text_from_json2(self: &Rc<Self>, values: &Vec<Root>) -> String {
        let mut out_str: String = String::new();
        for rdata in values {
            println!("{}", rdata.text);
            let aa = format!(" {}", rdata.text).clone();
            //let tmp_str = .as_str().clone();
            out_str += aa.as_str();
            //     outstr += data["text"];
        }
        return out_str;
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_add_clicked(self: &Rc<Self>) {
        QMessageBox::information_q_widget2_q_string(
            &self.ui.widget,
            &qs("请选择一张内容有文字的图片（jpg或png格式）"),
            &qs("功能提示：将选中图片的文字内容提取并在列表中显示。"),
        );
        //// 将当前时间字符串添加到 listwidget
        // self.ui
        //     .list_widget
        //     .add_item_q_string(&QDateTime::current_date_time().to_string());

        //打开文件对话框，选择图片。用于paddleOCR文字识别：
        let filename = QFileDialog::get_open_file_names_0a();

        let ocr_result = ocr::use_ocr(qs(filename.at(0).to_std_string()));
        let result_json = serde_json::from_str::<serde_json::Value>(ocr_result.as_str()).unwrap();

        //这是 Root Object，其数据内容例子：Root { box_points: [BoxPoint { x: 1097, y: 476 }, BoxPoint { x: 1229, y: 476 }, BoxPoint { x: 1216, y: 1282 }, BoxPoint { x: 1085, y: 1282 }], score: 0.939254343509674, text: "金额（大写）" }
        let result = serde_json::from_str::<OcrResult>(&result_json["data"].to_string()).unwrap();
        println!("转换后的结果：{:?}", result);
        let result_clone = result.clone();
        println!("内部数据（第一组数据）：{}", result[0].text);

        //使用函数提取所有 text 字段合并成 一个字符串
        let all_result = self.get_ocr_text_from_json2(&result_clone);
        println!(
            "该 {} 图片中的所有文字内容： {}",
            filename.at(0).to_std_string(),
            all_result
        );

        let qstr_all_result = QString::from_std_str(all_result);
        //println!("{:?}", qstr_all_result); //这里打印结果是 QString的指针值，并非QString的字符串内容。需按下面as_ref()获得QString内容。
        self.ui
            .list_widget
            .add_item_q_string(qstr_all_result.as_ref()); //这是rust_qt 的 CppBox<QString>转 QString 的方法。
                                                          //.add_item_q_string(qt_core::QString { all_result });
    }
}


/// may also write `CRUDTable` as `impl CRUDTable for BizActivity{}`
/// #[crud_table]
/// #[crud_table(table_name:biz_activity)]
/// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag")]
/// #[crud_table(table_name:"biz_activity"|table_columns:"id,name,version,delete_flag"|formats_pg:"id:{}::uuid")]
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

pub async fn mytest() {
    /// enable log crate to show sql logs
    //fast_log::init(fast_log::config::Config::new().console());
    /// initialize rbatis. May use `lazy_static` crate to define rbatis as a global variable because rbatis is thread safe
    let rb = rbatis::rbatis::Rbatis::new();
    println!("in mydb_test func1");
    /// connect to database  
    let linkstatus = rb.link("sqlite://sqlite3.db").await;
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
    let results: Vec<Posts> = rb.fetch_list().await.unwrap();
    //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1
    println!("所有记录： {:?}", results);

    /*
    ///query by id vec
    let result: Vec<Posts> = rb.list_by_column("id", &["1"]).await.unwrap();
    //Query ==> SELECT create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id IN  (?)

    ///query by wrapper
    let r: Result<Option<Posts>, Error> = rb.fetch_by_wrapper(rb.new_wrapper().eq("id", "1")).await;
    //Query ==> SELECT  create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version  FROM biz_activity WHERE delete_flag = 1  AND id =  ?

    ///delete
    rb.remove_by_column::<Posts, _>("id", &"1").await;
    //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id = 1

    ///delete batch
    rb.remove_batch_by_column::<Posts, _>("id", &["1", "2"])
        .await;
    //Exec ==> UPDATE biz_activity SET delete_flag = 0 WHERE id IN (  ?  ,  ?  )

    ///update
    let mut activity = activity.clone();
    let r = rb.update_by_column("id", &activity).await;
    //Exec   ==> update biz_activity set  status = ?, create_time = ?, version = ?, delete_flag = ?  where id = ?
    rb.update_by_wrapper(
        &activity,
        rb.new_wrapper().eq("id", "12312"),
        &[Skip::Value(&serde_json::Value::Null), Skip::Column("id")],
    )
    .await;
    //Exec ==> UPDATE biz_activity SET  create_time =  ? , delete_flag =  ? , status =  ? , version =  ?  WHERE id =  ?
    */
    //let ret: i32 = 42;
    //future::ok(ret)
}

#[tokio::main]
pub async fn main() {
    /// customize connection pool parameters (optional)
    // let mut opt =PoolOptions::new();
    // opt.max_size=100;
    // rb.link_opt("mysql://root:123456@localhost:3306/test",&opt).await.unwrap();
    /// newly constructed wrapper sql logic
    ///
    ///
    /*
    let wrapper = rb.new_wrapper()
            .eq("id", 1)                    //sql:  id = 1
            .and()                          //sql:  and
            .ne(BizActivity::id(), 1)       //sql:  id <> 1
            .in_array("id", &[1, 2, 3])     //sql:  id in (1,2,3)
            .not_in("id", &[1, 2, 3])       //sql:  id not in (1,2,3)
            .like("name", 1)                //sql:  name like 1
            .or()                           //sql:  or
            .not_like(BizActivity::name(), "asdf")       //sql:  name not like 'asdf'
            .between("create_time", "2020-01-01 00:00:00", "2020-12-12 00:00:00")//sql:  create_time between '2020-01-01 00:00:00' and '2020-01-01 00:00:00'
            .group_by(&["id"])              //sql:  group by id
            .order_by(true, &["id", "name"])//sql:  group by id,name
            ;

      let activity = BizActivity {
          id: Some("12312".to_string()),
          name: None,
          pc_link: None,
          h5_link: None,
          pc_banner_img: None,
          h5_banner_img: None,
          sort: None,
          status: None,
          remark: None,
          create_time: Some(rbatis::DateTimeNative::now()),
          version: Some(1),
          delete_flag: Some(1),
      };
      /// saving
      rb.save(&activity, &[]).await;
      //Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

      /// batch saving
      rb.save_batch(&vec![activity], &[]).await;
      //Exec ==> INSERT INTO biz_activity (create_time,delete_flag,h5_banner_img,h5_link,id,name,pc_banner_img,pc_link,remark,sort,status,version) VALUES ( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? ),( ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? , ? )

      */
    mytest().await;
    QApplication::init(|app| unsafe {
        //根据 https://github.com/jnbooth/ruic 的说法，
        //in Rust+Qt, the way to create a parentless widget is to pass NullPtr as the parent.
        let _form = HelloWorld::new(NullPtr);
        _form.ui.btn_quit.clicked().connect(app.slot_quit());
        _form.show();
        QApplication::exec()
    })
}
