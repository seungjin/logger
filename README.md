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


## How to call  
```consle
# curl https://logger.seungjin.net \
  -H 'AUTHKEY: MY_AUTHKEY_HERE' \
  -H 'Content-Type: application/json' \
  -d $(echo "{\"hostname\": \"$(hostname)\", \"ip\": \"$(curl -s ifconfig.io)\"}" | jq -c)

# curl "https://logger.seungjin.net/$(hostname -f | tr -d ' \n')/ip" \ 
  -H 'AUTHKEY: MY_AUTHKEY_HERE' \
  -H 'Content-Type: application/json' \
  -d $(ip --json a)
```



  
