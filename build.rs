use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    // 获取构建模式
    let profile = env::var("PROFILE").unwrap();

    // 确定目标目录
    let target_dir = match profile.as_str() {
        "debug" => "debug",
        "release" => "release",
        _ => panic!("Unknown build profile: {}", profile),
    };

    // 拷贝资源到目标目录
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dest_path = current_dir.join("target").join(target_dir);
    fs::create_dir_all(&dest_path).unwrap();
    copy_directory(Path::new("./demo_images/"), &dest_path.join("demo_images"))?;
    copy_directory(
        Path::new("./PaddleOCR-json/"),
        &dest_path.join("PaddleOCR-json"),
    )?;
    Ok(())
}

fn copy_directory(src: &Path, dst: &Path) -> io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let entry_path = entry.path();
            let file_name = entry_path.file_name().unwrap();
            let dst_path = dst.join(file_name);

            if entry_path.is_dir() {
                copy_directory(&entry_path, &dst_path)?;
            } else {
                fs::copy(&entry_path, &dst_path)?;
            }
        }
    }
    Ok(())
}
