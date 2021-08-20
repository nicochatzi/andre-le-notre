FROM resin/rpi-raspbian:latest
ENTRYPOINT []

# RUN mkdir /app
# COPY . /app
# WORKDIR /app

RUN sudo apt-get install python3-gpiozero && \
    sudo apt-get install python3-distutils

CMD ["python3", "src/main.py"]
