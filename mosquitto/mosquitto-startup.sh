#!/bin/sh
mkdir -p /mosquitto/data /mosquitto/log
chown -R mosquitto:mosquitto /mosquitto
chmod -R 755 /mosquitto
chmod 700 /mosquitto/data/mosquitto.db