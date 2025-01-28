import paho.mqtt.client as mqtt
import time
import random
import sys

# Define the MQTT broker address and topic
broker_address = "mosquitto"  # Use "localhost" if running on the same machine
port = 1883
protocol = mqtt.MQTTv5
callback_api_version = mqtt.CallbackAPIVersion.VERSION2
client_id = "sensor"
topic = "homestead/" + client_id

def measure_cpu_temp():
    # temp = os.popen("vcgencmd measure_temp").readline() # in raspberry pi
    # return float(temp.replace("temp=","").replace("'C",""))
    return random.uniform(-5,40)

def measure_cpu_voltage():
    # volts = os.popen("vcgencmd measure_volts").readline() # in raspberry pi
    # return float(volts.replace("volt=","").replace("V",""))
    return random.uniform(1,10)

def create_client():
    def on_connect(client, userdata, flags, reason_code, properties):
        if reason_code == 0:
            print("🟢 "+client_id+" connected to MQTT")
        if reason_code == "Unsupported protocol version":
            print("Unsupported protocol version")
        if reason_code == "Client identifier not valid":
            print("Client identifier not valid")
    def on_disconnect(client, userdata, flags, reason_code, properties):
        if reason_code == 0:
            print("Disconnected successfully")
        if reason_code > 0:
            print("Error while disconnecting")

    # Create a new MQTT client instance
    client = mqtt.Client(client_id=client_id, protocol=protocol, callback_api_version=callback_api_version)
    client.on_connect = on_connect
    client.on_disconnect = on_disconnect
    return client

client = create_client()
client.loop_start()
try:
    print("🟠 "+client_id+" connecting to MQTT...")
    client.connect(broker_address,port)
except:
    print("🔴 "+client_id+" failed to connect to MQTT!")
    sys.exit(1)


try:
    while True:
        src_timestamp = int(time.time()*1000)
        cpu_temp = str(measure_cpu_temp())
        cpu_volt = str(measure_cpu_voltage())
        payload = {
            "src_timestamp": src_timestamp,
            "cpu_temp": cpu_temp,
            "cpu_volt": cpu_volt
        }
        client.publish(topic, str(payload))
        print(f"Published {payload} to {topic}")
        time.sleep(30)  # Wait for 5 seconds before publishing the next message
except KeyboardInterrupt:
    print("Exiting...")
finally:
    client.loop_stop()
    client.disconnect()