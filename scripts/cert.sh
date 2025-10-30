#!/bin/bash
set -e

echo "Renew certification..."

# renew the cert
echo "Execute the renew command..."
docker run --rm \
	-v /home/attorneys-website/data/certbot/conf:/etc/letsencrypt \
	-v /home/attorneys-website/data/certbot/www:/var/www/certbot \
	certbot/certbot renew --quiet


# reload the nginx to get the new cert
echo "Reload nginx..."
docker exec nginx nginx -s reload
