FROM postgres
ENV POSTGRES_PASSWORD postgres
COPY *.sql /docker-entrypoint-initdb.d/
EXPOSE 5432/tcp
