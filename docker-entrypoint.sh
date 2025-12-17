#!/bin/sh
set -e

# Start backend in background
splitdumb serve --port 3000 --data-file /data/app_data.json &

# Start nginx in foreground
nginx -g 'daemon off;'
