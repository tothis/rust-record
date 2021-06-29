use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;

use ssh2::Session;

const FILE_PATH: &str = "Cargo.toml";

fn main() {
    let tcp = TcpStream::connect("192.168.1.128:22").unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    session.userauth_password("root", "123456").unwrap();
    session.authenticated();

    let mut channel = session.channel_session().unwrap();

    // 执行命令并打印输出
    channel.exec("ls").unwrap();
    let mut ls = String::new();
    channel.read_to_string(&mut ls).unwrap();
    println!("{}", ls);
    channel.wait_close().unwrap();

    // 上传文件
    let result = fs::read(FILE_PATH).unwrap();
    let mut remote_file = session.scp_send(Path::new(FILE_PATH)
                                           , 0o644, result.len() as u64, None).unwrap();
    remote_file.write(&result).unwrap();

    // 下载文件
    let (mut remote_file, _) = session.scp_recv(Path::new(FILE_PATH))
        .unwrap();
    let mut read = Vec::new();
    remote_file.read_to_end(&mut read).unwrap();
    println!("{:?}", String::from_utf8(read).unwrap());

    // 关闭频道，等待全部内容传输完毕
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
}
