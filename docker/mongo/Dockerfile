FROM mongo:3.6.17 AS production-pseudo

# --

FROM production-pseudo AS development

COPY etc/mongod.conf /etc/mongod.conf
COPY setup.sh /docker-entrypoint-initdb.d/setup.sh
