#!/bin/bash

echo "Renew certification..."

CMD="docker run -it --rm -v /home/attorneys-website/data/certbot/conf:/etc/letsencrypt -v /home/attorneys-website/data/certbot/www:/var/www/certbot certbot/certbot renew"

RELOAD_NGINX_CMD="docker exec nginx nginx -s reload"

# renew the cert
echo "Execute the renew command..."
$CMD

# reload the nginx to get the new cert
echo "Reload the nginx..."
$RELOAD_NGINX_CMD
