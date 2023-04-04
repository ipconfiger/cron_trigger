# cron_trigger
定时执行任务，如果执行失败，则会通过配置的email地址或者通过webhook发出通知
Run cron task, send notification via email or webhook if it goes wrong

### 使用-Usage
```
./cron_trigger
error: The following required arguments were not provided:
--cron <CRON FILE>
--notifications <NOTIFY_FILE>

USAGE:
  cron_trigger --cron <CRON FILE> --notifications <NOTIFY_FILE>

For more information try --help
```
### 配置-Configuration

参数cron 传入一个类crontab的配置文件，区别是crontab的精度是分钟，这个配置精度是秒
       Pass in a crontab-like configuration file, the difference is that the precision of crontab is minutes, and the precision of this configuration is seconds.

```
1/10 * * * * * *  /bin/sh /opt/scripts/check_service.sh  # 每十秒检测一次服务运行状态 Check Service Every 10 seconds
1 * * * * * *     /bin/sh /opt/scripts/check_disk.sh     # 每分钟第一秒执行磁盘检测 Check Disk Every minites
```

参数notifications 一个yml配置文件，用于配置发email或者webhook所需参数 A yml configuration file used to configure the parameters required for sending emails or webhooks.

```
smtp: "smtp server ip or domain"
smtp_port: 25
from_addr: "xxx@yyy.com"
account: "xxx@yyy.com"
pwd: "*********"
starttls: false
nlist:
  - item_type: "mail"
    address: "23826299@qq.com"
  - item_type: "web"
    address: "http://127.0.0.1:8878/send/sms"
```

## 带解决问题 ISSUSE
lettre库在使用target=x86_64-unknown-linux-musl 编译后，运行到starttls_relay的时候会panic



