import paho.mqtt.client as mqtt
import time
import sys
from paho.mqtt.packettypes import PacketTypes
import json
import uuid

# Define the MQTT broker address and topic
broker_address = "mosquitto"  # Use "localhost" if running on the same machine
port = 1883
protocol = mqtt.MQTTv5
callback_api_version = mqtt.CallbackAPIVersion.VERSION2
client_id = "actuator"
topic = "homestead/"+client_id

connected = False

state = False

def handle_get_state(message):
    # Get the response properties, abort if they're not given
    props = message.properties
    if not hasattr(props, 'ResponseTopic') or not hasattr(props, 'CorrelationData'):
        print("No reply requested")
        return

    corr_id = props.CorrelationData
    reply_to = props.ResponseTopic

    print("Sending response "+str(state)+" on '"+reply_to+"': "+str(corr_id))
    props = mqtt.Properties(PacketTypes.PUBLISH)
    props.CorrelationData = corr_id
    payload = json.dumps(state)
    client.publish(reply_to, payload, qos=1, properties=props)

def handle_set_state(message):
    global state
    msg = str(message.payload.decode("utf-8","ignore"))
    if msg == "true":
        state = True
    elif msg == "false":
        state = False
    # Get the response properties, abort if they're not given
    props = message.properties
    if not hasattr(props, 'ResponseTopic') or not hasattr(props, 'CorrelationData'):
        print("No reply requested")
        return

    corr_id = props.CorrelationData
    reply_to = props.ResponseTopic

    print("Sending response "+str(state)+" on '"+reply_to+"': "+str(corr_id))
    props = mqtt.Properties(PacketTypes.PUBLISH)
    props.CorrelationData = corr_id
    payload = json.dumps(state)
    client.publish(reply_to, payload, qos=1, properties=props)


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
    def on_subscribe(client, userdata, mid, reason_code_list, properties):
        print("Actuator subscribed: "+str(mid))
    def on_message(client, userdata, message):
        msg = str(message.payload.decode("utf-8","ignore"))
        print(message.topic+" | Message received: " + msg)
        print("Properties:", message.properties)

        if message.topic == "homestead/actuator/get_state":
            handle_get_state(message)
        elif message.topic == "homestead/actuator/set_state":
            handle_set_state(message)

        
    

    client = mqtt.Client(client_id=client_id, protocol=protocol, callback_api_version=callback_api_version)
    client.on_connect = on_connect
    client.on_message = on_message
    client.on_subscribe = on_subscribe
    client.on_disconnect = on_disconnect
    return client

client = create_client()
try:
    print("🟠 "+client_id+" connecting to MQTT...")
    client.connect(broker_address,port)
except:    
    print("🔴 "+client_id+" failed to connect to MQTT!")
    sys.exit(1)

client.subscribe([("homestead/actuator/get_state", 1), ("homestead/actuator/set_state", 1)])

# request_topic = "homestead/actuator/get_state"
# response_topic = f"response/{uuid.uuid4()}"
# publish_properties = mqtt.Properties(mqtt.PacketTypes.PUBLISH)
# publish_properties.ResponseTopic = response_topic
# publish_properties.CorrelationData = str(uuid.uuid4()).encode()
# publish_properties.UserProperty = [("MessageType", "DeviceCreationRequest")]
# payload = "This is a request message"
# client.publish(request_topic, payload=payload, qos=1, properties=publish_properties)

client.loop_forever()
