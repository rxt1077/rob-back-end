#!/bin/bash

docker run -d -p 5432:5432 -e POSTGRES_PASSWORD="postgres" postgres
psql -h localhost -U postgres < ../sql/init.sql
