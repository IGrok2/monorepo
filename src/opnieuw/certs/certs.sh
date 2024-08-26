rm *.pem
rm *.srl
rm *.cnf

# 1. Generate CA's private key and self-signed certificate
openssl req -x509 -newkey rsa:4096 -days 3600 -nodes -keyout ca-key.pem -out ca-cert.pem -subj "/C=US/ST=Come/L=Work/O=Here
/OU=Dope/CN=*/emailAddress=edward@packetware.net"

echo "CA's self-signed certificate"
openssl x509 -in ca-cert.pem -noout -text

# 2. Generate web server's private key and certificate signing request (CSR)
openssl req -newkey rsa:4096 -nodes -keyout server-key.pem -out server-req.pem -subj "/C=US/ST=Come/L=Work/O=Here
/OU=Dope/CN=*/emailAddress=edward@packetware.net"

# Remember that when we develop on localhost, It’s important to add the IP:0.0.0.0 as an Subject Alternative Name (SAN) extension to the certificate.
echo "subjectAltName=IP:185.199.95.181,IP:104.129.132.111,IP:195.60.167.241,IP:168.100.160.167,IP:54.81.88.169,IP:192.168.86.32,IP:192.168.86.41,IP:185.199.94.66,IP:5.161.57.23,IP:185.199.94.67" > server-ext.cnf
# Or you can use localhost DNS and grpc.ssl_target_name_override variable
# echo "subjectAltName=DNS:localhost" > server-ext.cnf

# 3. Use CA's private key to sign web server's CSR and get back the signed certificate
openssl x509 -req -in server-req.pem -days 3600 -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -out server-cert.pem -extfile server-ext.cnf

echo "Server's signed certificate"
openssl x509 -in server-cert.pem -noout -text

# 4. Generate client's private key and certificate signing request (CSR)
openssl req -newkey rsa:4096 -nodes -keyout client-key.pem -out client-req.pem -subj "/C=US/ST=Come/L=Work/O=Here
/OU=Dope/CN=*/emailAddress=edward@packetware.net"

# Remember that when we develop on localhost, It’s important to add the IP:0.0.0.0 as an Subject Alternative Name (SAN) extension to the certificate.
echo "subjectAltName=DNS:*.client.com,IP:0.0.0.0" > client-ext.cnf

# 5. Use CA's private key to sign client's CSR and get back the signed certificate
openssl x509 -req -in client-req.pem -days 60 -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -out client-cert.pem -extfile client-ext.cnf

echo "Client's signed certificate"
openssl x509 -in client-cert.pem -noout -text
