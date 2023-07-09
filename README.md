# BookManager

![screenshot](screenshot.png)

## 运行

请按照以下步骤在您的系统上运行BookManager：

1. **安装Rust：** BookManager是基于Rust构建的。如果您的系统中还未安装Rust，可以从[官方网站](https://www.rust-lang.org/)下载。

2. **安装SQLite3：** BookManager使用SQLite3进行数据库管理。如果您的系统中还未安装SQLite3，可以从[SQLite3官方网站](https://www.sqlite.org/index.html)下载并按照其提供的指南进行安装。

3. **配置环境变量：** 导航到`.env`文件，修改`DATABASE_URL`和`CERTIFICATE_DIR`变量，使它们分别指向您选择的数据库和证书目录。

4. **启动服务器：** 在终端（Linux系统）或命令提示符/PowerShell（Windows系统）中运行以下命令启动服务器：
```
cargo run
```

5. **访问BookManager：** 打开您的首选网络浏览器，访问[localhost:8080](http://localhost:8080)来使用BookManager。
