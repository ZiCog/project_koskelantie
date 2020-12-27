#!/bin/bash
#
# serve.sh
# Runs miniserve web server for testing project_kokelantie web app.

miniserve -p8081 ./static --index index.html
