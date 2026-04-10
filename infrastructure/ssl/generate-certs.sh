# Self-Signed SSL Certificate Generation Script
# For development/testing only - replace with real certificates in production

# Generate private key
openssl genrsa -out server.key 2048

# Generate self-signed certificate (valid for 1 year)
openssl req -new -x509 -key server.key -out server.crt -days 365 \
    -subj "/C=KE/ST=Nairobi/L=Nairobi/O=SmartLMS/OU=IT/CN=*.smartlms.io"

# Optional: Generate combined PEM (key + cert) for some servers
cat server.key server.crt > server.pem

echo "SSL certificates generated for development use."
echo "WARNING: Replace with real certificates before production deployment!"
echo ""
echo "Files created:"
echo "  - server.key  (private key - keep secret!)"
echo "  - server.crt  (self-signed certificate)"
echo "  - server.pem  (combined key + cert)"