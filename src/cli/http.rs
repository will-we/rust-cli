use clap::Parser;

#[derive(Parser, Debug)]
pub enum HttpCommand {
    #[command(name = "serve", about = "服务端")]
    Serve(ServeOpts),
    #[command(name = "decode", about = "客户端")]
    Client(ClientOpts),
}

#[derive(Parser, Debug)]
pub struct ServeOpts {
    #[arg(short, long, default_value = "8080", help = "监听端口")]
    pub port: u16,
    #[arg(long, default_value = ".", help = "文件根目录")]
    pub path: String,
}

#[derive(Parser, Debug)]
pub struct ClientOpts {
    #[arg(short, long, help = "服务端地址")]
    pub server: String,
    #[arg(short, long, help = "服务端端口")]
    pub port: u16,
    #[arg(short, long, help = "请求路径")]
    pub path: String,
    #[arg(short, long, help = "请求方法")]
    pub method: String,
    #[arg(short, long, help = "请求头")]
    pub headers: Vec<String>,
}
