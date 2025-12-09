## Crear migracion
1. sqlx migrate add update_links_timezone_cancun 
2. en el archivo nuevo dentro de migrations mete tu script
3. sqlx migrate run --database-url sqlite:qr.db 

## Query tables
- sqlite3 qr.db "SELECT * FROM links;" 
- sqlite3 qr.db "SELECT * FROM scans;" 


## Post
curl -X POST http://localhost:3000/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://cntnt.xplaya.com"}'
