# DynDns53
[![license](https://img.shields.io/github/license/cgrotz/dyndns53.svg)](https://github.com/cgrotz/dyndns53/blob/master/LICENSE)
[![release](https://img.shields.io/github/release/cgrotz/dyndns53.svg)](https://github.com/cgrotz/dyndns53/releases/latest)
![Continuous integration](https://github.com/cgrotz/dyndns53/workflows/Continuous%20integration/badge.svg)
[![CodeFactor](https://www.codefactor.io/repository/github/cgrotz/dyndns53/badge)](https://www.codefactor.io/repository/github/cgrotz/dyndns53)

A simple tool, to update a DNS entry in Route53 based on your current WAN IP address

## Usage
Simply call the executable dyndns53 and provide as first parameter the ID of your hosted zone in Route53 second parameter is the domain you want to use with a trailing point for example dyn.mydomain.org.

## Installation as a systemd service on Ubuntu
You can download the latest release for unix using the following shell sequence

    curl -o /usr/bin/dyndns53 https://github.com/cgrotz/dyndns53/releases/download/<version>/dyndns53-x86_64-unknown-linux-gnu
    chmod a+x /usr/bin/dyndns53

### Edit /etc/systemd/system/update-ip.service
    [Unit]
    Description=Update IP on route53
    Wants=update-ip.timer

    [Service]
    Environment="AWS_ACCESS_KEY_ID=<Your AccessKey>"
    Environment="AWS_SECRET_ACCESS_KEY=<Your Access Key Secret>"
    Environment="AWS_DEFAULT_REGION=eu-central-1"
    ExecStart=/usr/bin/dyndns53
    WorkingDirectory=/tmp

    [Install]
    WantedBy=multi-user.target

### Edit /etc/systemd/system/update-ip.timer
    [Unit]
    Description=Run update-ip every 60 minutes
    Requires=update-ip.service

    [Timer]
    Unit=update-ip.service
    OnUnitInactiveSec=60m
    RandomizedDelaySec=60m
    AccuracySec=1s

    [Install]
    WantedBy=timers.target

### Update Systemd
    systemctl daemon-reload
    systemctl enable update-ip.service update-ip.timer
