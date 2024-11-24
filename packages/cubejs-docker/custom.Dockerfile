FROM cubejs/cube:v1.1.0

RUN apt update && apt install -y python3-pip && rm -rf /var/lib/apt/lists/* /var/cache/apt/*

RUN pip3 install pyyaml==6.0.1 --break-system-packages
