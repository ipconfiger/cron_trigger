#!/bin/sh

sudo echo "1 * * * * * *   /bin/date\n" > /etc/cron_tasks
if [ $? -ne 0 ];then
   echo "no permision!"
   exit $?
fi
sudo rm -f /etc/notify_configure.yml
echo "smtp: \"smtp server ip or domain\"\n" >> /etc/notify_configure.yml
echo "smtp_port: 25\n" >> /etc/notify_configure.yml
echo "from_addr: \"xxx@yyy.com\"\n" >> /etc/notify_configure.yml
echo "account: \"xxx@yyy.com\"" >> /etc/notify_configure.yml
echo "pwd: \"*********\"\n" >> /etc/notify_configure.yml
echo "nlist:\n" >> /etc/notify_configure.yml
echo "  - item_type: \"mail\"\n" >> /etc/notify_configure.yml
echo "    address: \"23826299@qq.com\"\n" >> /etc/notify_configure.yml
echo "  - item_type: \"web\"\n" >> /etc/notify_configure.yml
echo "    address: \"http://127.0.0.1:8878/send/sms\"\n" >> /etc/notify_configure.yml

sudo cp cron_trigger /usr/local/bin
sudo rm -f /usr/lib/systemd/system/cron_trigger.service
echo "[Unit]\n" >> ./cron_trigger.service
echo "Description=cron_trigger\n" >> ./cron_trigger.service
echo "\n" >> ./cron_trigger.service
echo "[Service]\n" >> ./cron_trigger.service
echo "ExecStart=/usr/local/bin/cron_trigger --cron /etc/cron_tasks --notifications /etc/notify_configure.yml\n" >> ./cron_trigger.service
echo "Restart=always\n" >> ./cron_trigger.service
echo "\n" >> ./cron_trigger.service
echo "[Install]\n" >> ./cron_trigger.service
echo "WantedBy=multi-user.target\n" >> ./cron_trigger.service
sudo mv ./cron_trigger.service /usr/lib/systemd/system/
sudo systemctl enable cron_trigger.service
sudo systemctl restart cron_trigger

