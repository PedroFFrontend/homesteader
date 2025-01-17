import paho.mqtt.client as mqtt
import time
import random

# Define the MQTT broker address and topic
broker_address = "mosquitto"  # Use "localhost" if running on the same machine
port = 1883
topic = "homestead/cpu"

print("starting sensors",{broker_address,port,topic})


def measure_cpu_temp():
    # temp = os.popen("vcgencmd measure_temp").readline() # in raspberry pi
    # return float(temp.replace("temp=","").replace("'C",""))
    return random.uniform(-5,40)

def measure_cpu_voltage():
    # volts = os.popen("vcgencmd measure_volts").readline() # in raspberry pi
    # return float(volts.replace("volt=","").replace("V",""))
    return random.uniform(1,10)


# Connect to the broker
def connect_mqtt():
    def on_connect(client, userdata, flags, reason_code, properties):
        if reason_code == "Unsupported protocol version":
            # handle bad protocol version
            print("Unsupported protocol version")
        if reason_code == "Client identifier not valid":
            # handle bad identifier
            print("Client identifier not valid")
    # NEW code for both version
    def on_disconnect(client, userdata, flags, reason_code, properties):
        if reason_code == 0:
            # success disconnect
            print("Disconnected successfully")
        if reason_code > 0:
            # error processing
            print("Error while disconnecting")

    # Create a new MQTT client instance
    client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2, "Sensor")
    client.on_connect = on_connect
    client.on_disconnect = on_disconnect
    client.connect(broker_address,port)
    return client

print("connecting mqtt")
client = connect_mqtt()
print("mqtt connected")

# Publish messages in a loop
try:
    while True:
        cpu_temp = str(measure_cpu_temp())
        cpu_volt = str(measure_cpu_voltage())
        payload = {
            "src_timestamp": int(time.time()*1000),
            "cpu_temp": cpu_temp,
            "cpu_volt": cpu_volt
        }
        client.publish(topic, str(payload))
        print(f"Published {payload} to {topic}")
        time.sleep(30)  # Wait for 5 seconds before publishing the next message
except KeyboardInterrupt:
    print("Exiting...")
finally:
    client.disconnect()
