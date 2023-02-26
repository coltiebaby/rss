# LCU

If you want to make your own wrapper

### Lockfile
/Applications/League\ of\ Legends.app/Contents/LoL/lockfile

```
# --install-directory
# `ps x -o args | grep 'LeagueClientUx'`

$ ps -A | grep "RiotClientUx"

# View swagger
$ curl -u riot:your-token-here --basic --insecure https://localhost:56970/swagger/v1/api-docs
```
