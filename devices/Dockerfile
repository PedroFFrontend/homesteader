FROM python:3.10.16-alpine

COPY . .
COPY run_all.sh .

RUN pip install --upgrade --no-cache-dir -r requirements.txt

CMD ["sh", "run_all.sh"]
