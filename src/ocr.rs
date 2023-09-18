use paddleocr;
//use qt_core::qs;
use cpp_core::CppBox;
use qt_core::QString;

pub fn use_ocr(from_img: CppBox<QString>)->String
{
    let mut p = paddleocr::Ppocr::new(std::path::PathBuf::from(
        "./PaddleOCR-json/PaddleOCR_json.exe", // path to binary
    )).unwrap(); // initialize
    
    let now = std::time::Instant::now(); // benchmark
    // OCR files
    let ocr_result = p.ocr(from_img.to_std_string()).unwrap();
    println!("{}", ocr_result);
    //println!("{}", p.ocr(".../test2.png").unwrap());
    //println!("{}", p.ocr(".../test3.png").unwrap());

    // OCR clipboard
    //println!("{}", p.ocr_clipboard().unwrap());

    
    println!("Elapsed: {:.2?}", now.elapsed());
    return ocr_result;
}
