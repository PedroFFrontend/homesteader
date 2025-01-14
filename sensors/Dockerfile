FROM python:3.10.16-alpine
# WORKDIR /app
# COPY package*.json ./
# RUN npm install
# COPY . .
# RUN npm run build
# EXPOSE 3000
COPY . .
RUN pip install --upgrade pip
RUN pip install --upgrade paho-mqtt

CMD ["python3", "sensor.py"]