#![windows_subsystem = "console"] //要显示控制台时，仅启用这句
                                  //#![windows_subsystem = "windows"]  // 要隐藏时，仅启用这句

use cpp_core::{CastInto, NullPtr, Ptr, StaticUpcast};
use qt_core::{qs, slot, QDateTime, QObject, QString, SlotNoArgs};
use qt_widgets::{QApplication, QFileDialog, QMessageBox, QWidget};
use serde_json;
use std::{rc::Rc, str::FromStr};
mod ocr;
mod uic;

use serde::{Deserialize, Serialize};
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

fn main() {
    QApplication::init(|app| unsafe {
        //根据 https://github.com/jnbooth/ruic 的说法，
        //in Rust+Qt, the way to create a parentless widget is to pass NullPtr as the parent.
        let _form = HelloWorld::new(NullPtr);
        _form.ui.btn_quit.clicked().connect(app.slot_quit());
        _form.show();
        QApplication::exec()
    })
}
