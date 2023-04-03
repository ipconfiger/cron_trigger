use tokio::time;
use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use std::panic;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use std::fs;
use serde::{Deserialize, Serialize};
use clap::{App, Arg};
use reqwest::blocking::Client;
use std::collections::HashMap;

fn send_web(to_addr: String, content: String) {
    let mut form_data = HashMap::new();
    form_data.insert("error", content);

    let client = Client::new();
    let result = client.post(to_addr)
        .form(&form_data)
        .send();
    match result {
        Ok(response)=>{
            println!("web sent:{}", response.text().unwrap());
        },
        Err(ex)=>{
            println!("web sent fait:{}", ex);
        }
    }
}

fn send_mail(config: &Config, to_addr: String, title: String, content: String) {
    // 设置发件人邮箱地址
    let from = config.from_addr.parse().expect("Invalid from addr");
    // 设置收件人邮箱地址
    let to = to_addr.parse().expect("Invalid target addr");
    // 创建邮件
    println!("creating mail");
    let email = Message::builder()
        .from(from)
        .to(to)
        .subject(title)
        .header(ContentType::TEXT_PLAIN)
        .body(content)
        .expect("Invalid mail");
    // 设置SMTP服务器地址和端口号
    let smtp_server = config.smtp.clone();
    let smtp_port = config.smtp_port;
    // 设置发件人邮箱账号和密码
    let username = config.account.clone();
    let password = config.pwd.clone();
    println!("creating smtp transport");
    // 创建SMTP传输对象
    let smtp_result = SmtpTransport::starttls_relay(smtp_server.as_str());
    match smtp_result {
        Ok(smtp_transport)=>{
            let transport = smtp_transport.credentials(Credentials::new(username.to_string(), password.to_string()))
            .port(smtp_port)
            .build();
            println!("will send mail to:{}", to_addr);
            let result = transport.send(&email);
            match result {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => println!("Could not send email: {e:?}"),
            }

        }
        Err(ex)=>{
            println!("create transport faild with err:{ex:?}");
        }
    };
}


async fn task_that_takes_a_second(config_path: PathBuf, notify_path: PathBuf) {
    let mut task: Vec<String> = vec![];
    read_crontab_file(config_path, &mut task).await.unwrap();
    if task.len() > 0 {
        for command in task{
            println!("will execute task:{}", command);
            let notify_path = notify_path.clone();
            thread::spawn(move || {
                let args: Vec<&str> = command.split_whitespace().collect();
                let result = Command::new(args[0])
                    .args(&args[1..])
                    .output();
                match result {
                    Ok(output)=>{
                        println!("No problem!")
                    },
                    Err(ex)=>{
                        let notification_body = format!("cmd:{} with Err:{}", command, ex);
                        // println!("will send:{}", notification_body);
                        send_notification(notify_path, "Test Alarm", notification_body.as_str());
                    }
                }
                
            });
        }
    }
}

fn get_abs_path(address: &String) -> PathBuf {
    let absolute_path = if Path::new(address).is_relative() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let mut path_buf = PathBuf::from(current_dir);
        path_buf.push(address);
        path_buf
    } else {
        PathBuf::from(address)
    };
    return absolute_path;
}


#[derive(Debug, Deserialize, Serialize)]
struct Config {
    smtp: String,
    smtp_port: u16,
    account: String,
    pwd: String,
    from_addr: String,
    nlist: Vec<ListItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ListItem {
    item_type: String,
    address: String,
}

fn read_notification_config(config_path: PathBuf) -> Config {
    let contents = fs::read_to_string(config_path).expect("File Not Exists");
    let config: Config = serde_yaml::from_str(&contents).expect("Invalid Config");
    config
}

fn send_notification(config_path: PathBuf, title: &str, body: &str) {
    let config = read_notification_config(config_path);
    println!("load notification configure:{:?}", config);
    for item in &config.nlist {
        let tp = item.item_type.as_str();
        match tp {
            "web" =>{
                println!("get web hook, http post");
                send_web(item.address.clone(), body.to_string());
            },
            "mail"=>{
                println!("get mail hook, send mail");
                let result = panic::catch_unwind(|| {
                    send_mail(&config, item.address.clone(), title.to_string(), body.to_string());
                });
                if let Err(err) = result {
                    println!("Caught a panic: {:?}", err);
                }
            },
            _=>{}
        }
        
    }
    
}
 
#[tokio::main]
async fn main() {
    let matches = App::new("Cron Trigger Tests")
    .version("0.1")
    .author("Alexander.Li")
    .about("Trigger Tests Scripts in Cron")
    .arg(Arg::with_name("cron_file")
        .short('c')
        .long("cron")
        .value_name("CRON FILE")
        .help("CronTab configure file path")
        .required(true)
        .takes_value(true))
    .arg(Arg::with_name("notifications")
        .short('n')
        .long("notifications")
        .value_name("NOTIFY_FILE")
        .help("Notification method configure")
        .required(true)
        .takes_value(true))
    .get_matches();
    let address = matches.value_of("cron_file").unwrap();
    let notify_uri = matches.value_of("notifications").unwrap();
    
    let file_path = get_abs_path(&address.to_string());
    let notify_path = get_abs_path(&notify_uri.to_string());
    
    
    eprintln!("file path:{:?}", file_path);
    let mut interval = time::interval(time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        task_that_takes_a_second(file_path.clone(), notify_path.clone()).await;
    }
}

async fn read_crontab_file(config_path: PathBuf, tasks: &mut Vec<String>) -> io::Result<()> {
    let file = File::open(config_path).await?;
    let reader = BufReader::new(file);
    let now: DateTime<Utc> = Utc::now();
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        let time_config = line.split_whitespace().take(7).collect::<Vec<_>>().join(" ");
        if let Ok(cron) = cron::Schedule::from_str(&time_config) {
            if let Some(next) = cron.upcoming(Utc).next() {
                if (next.timestamp() - 1) == now.timestamp() {
                    let command = line.split_whitespace().skip(7).collect::<Vec<_>>().join(" ");
                    // execute command
                    println!("执行命令:{}", command);
                    tasks.push(command);
                }
            }
        }
    }
    Ok(())
}