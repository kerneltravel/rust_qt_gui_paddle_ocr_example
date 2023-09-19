use checksums::Algorithm;
use std::path::Path;

/// 计算文件md5值。
/// 用法 let file_md5 = filemd5::get_file_md5(std::path::Path::new("demo_images/1.jpg"));
/// println!("file_md5 {}", file_md5);
pub fn get_file_md5(path: &Path) -> String {
    return checksums::hash_file(path, Algorithm::MD5);
}
