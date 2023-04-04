#!/bin/sh

sudo echo "1 * * * * * *   /bin/date" > /etc/cron_tasks
if [ $? -ne 0 ];then
   echo "no permision!"
   exit $?
fi
sudo rm -f /etc/notify_configure.yml
echo "smtp: \"smtp server ip or domain\"" >> /etc/notify_configure.yml
echo "smtp_port: 25" >> /etc/notify_configure.yml
echo "from_addr: \"xxx@yyy.com\"" >> /etc/notify_configure.yml
echo "account: \"xxx@yyy.com\"" >> /etc/notify_configure.yml
echo "pwd: \"*********\"" >> /etc/notify_configure.yml
echo "starttls: false" >> /etc/notify_configure.yml
echo "nlist:" >> /etc/notify_configure.yml
echo "  - item_type: \"mail\"" >> /etc/notify_configure.yml
echo "    address: \"xxxx@xx.com\"" >> /etc/notify_configure.yml
echo "  - item_type: \"web\"" >> /etc/notify_configure.yml
echo "    address: \"http://127.0.0.1:8878/send/sms\"" >> /etc/notify_configure.yml

sudo cp cron_trigger /usr/local/bin
sudo rm -f /usr/lib/systemd/system/cron_trigger.service
echo "[Unit]" >> ./cron_trigger.service
echo "Description=cron_trigger" >> ./cron_trigger.service
echo "" >> ./cron_trigger.service
echo "[Service]" >> ./cron_trigger.service
echo "ExecStart=/usr/local/bin/cron_trigger --cron /etc/cron_tasks --notifications /etc/notify_configure.yml" >> ./cron_trigger.service
echo "Restart=always" >> ./cron_trigger.service
echo "" >> ./cron_trigger.service
echo "[Install]" >> ./cron_trigger.service
echo "WantedBy=multi-user.target" >> ./cron_trigger.service
sudo mv ./cron_trigger.service /usr/lib/systemd/system/
sudo systemctl enable cron_trigger.service
sudo systemctl restart cron_trigger

