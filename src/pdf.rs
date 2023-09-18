//use crate::path::{Path, PathBuf};
use pdf_extract;

/// 从指定的pdf中提取文本字符串
/// 使用方法：
/// let pdf_text =
/// pdf::extract_text_from_pdf_file(String::from_str("tests/docs/文字文稿1.pdf").unwrap());
/// println!("pdf 文本内容 {}", pdf_text);
pub fn extract_text_from_pdf_file(pdf_filepath: String) -> String {
    let bytes = std::fs::read(&pdf_filepath).unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    return out;
}
