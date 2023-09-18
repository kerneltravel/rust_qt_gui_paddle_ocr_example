
src/ocr.rs  文件的来源：

安装 ruic 工具，https://github.com/jnbooth/ruic
然后
```
cd src/
ruic.exe -o uic.rs --all mainwindow.ui 就会得到src/uic.rs

```

注意： ruic.exe 对 QT5 的Line 类 不支持。需要将uic.rs中重复的声明和 Line 相关的声明删掉。