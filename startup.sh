#/bin/sh

/usr/local/bin/dgraph alpha --lru_mb 1024 > /dev/null 2>&1 &
/usr/local/bin/dgraph zero > /dev/null 2>&1 &
/usr/local/bin/dgraph-ratel > /dev/null 2>&1 &
