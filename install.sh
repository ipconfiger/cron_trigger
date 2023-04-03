#!/bin/sh

sudo cp cron_trigger /usr/local/bin
echo "[Unit]\n" >> ./cron_trigger.service
echo "Description=cron_trigger\n" >> ./cron_trigger.service
echo "\n" >> ./cron_trigger.service
echo "[Service]\n" >> ./cron_trigger.service
echo "ExecStart=/usr/local/bin/cron_trigger\n" >> ./cron_trigger.service
echo "Restart=always\n" >> ./cron_trigger.service
echo "\n" >> ./cron_trigger.service
echo "[Install]\n" >> ./cron_trigger.service
echo "WantedBy=multi-user.target\n" >> ./cron_trigger.service
sudo mv ./cron_trigger.service /usr/lib/systemd/system/
sudo systemctl enable cron_trigger.service
sudo systemctl restart cron_trigger

