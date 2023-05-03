FROM mysql:8.0.29

ENV MYSQL_DATABASE veryrezsi

# All scripts in docker-entrypoint-initdb.d/ are automatically
# executed during container startup
COPY ./scripts/ /docker-entrypoint-initdb.d/
