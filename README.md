# Logger

## Service(Server) setting  
Use compose.yaml to launch container service.  
Set .env with  
```console
seungjin@free:~/apps/logger$ cat .env
LIBSQL_URL=libsql://iad-seungjin.turso.io
LIBSQL_TOKEN=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
AUTHKEY=XXXXXXXXXXXXXXXXXXXX
```

Currently being serviced at https://logger.seungjin.net  

free.gcp.seungjin.net / logger.seungjin.net  
/home/seungjin/apps/logger  

## Client  
<https://github.com/seungjin/logger-client>

## How to call with Curl  
```consle
# curl https://logger.seungjin.net \
  -H 'AUTHKEY: MY_AUTHKEY_HERE' \
  -H 'Content-Type: application/json' \
  -d $(echo "{\"hostname\": \"$(hostname)\", \"ip\": \"$(curl -s ifconfig.io)\"}" | jq -c)

# curl "https://logger.seungjin.net/$(hostname -f | tr -d ' \n')/ipa" \ 
  -H 'AUTHKEY: MY_AUTHKEY_HERE' \
  -H 'Content-Type: application/json' \
  -d $(ip --json a)
  
# curl "https://logger.seungjin.net/$(hostname -f | tr -d ' \n')/myip" \
  -H 'AUTHKEY: MY_AUTHKEY_HERE' \
  -H 'Content-Type: application/json' \
  -d "$(curl -s https://ifconfig.io/all.json)"
   
```

## auth.yaml sample   
```
# cat auth.yaml
---
general: THIS+IS+MY+KEY+FOR+GENERAL
app1: THIS+IS+KEY+FOR+APP1

```

## Database table schema  
```sql
CREATE TABLE message (
  sender TEXT,
  who TEXT,
  key TEXT,
  value TEXT,
  created_at DATETIME DEFAULT (strftime('%s', 'now'))
)
```



  
