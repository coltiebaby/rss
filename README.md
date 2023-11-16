# random skin selector

If you want to make your own wrapper

```
# --install-directory
# `ps x -o args | grep 'LeagueClientUx'`

$ ps -A | grep "RiotClientUx"

# << /Users/Shared/Riot\ Games/Riot\ Client.app/Contents/Frameworks/RiotClient.app/Contents/MacOS/RiotClientUx --app-port=12345 --remoting-auth-token=your-token-here --app-pid=3633 --log-dir=/log/dir/path --user-data-root=/user/data/root --app-root=/Users/Shared/Riot\ Games/Riot\ Client.app --crashpad-environment=KeystoneFoundationLiveMac

# View swagger
$ curl -u riot:your-token-here --basic --insecure https://localhost:56970/swagger/v1/api-docs

# create your websocket
$ curl --include \
-u riot:your-token-here --basic --insecure \
--no-buffer \
--header "Connection: Upgrade" \
--header "Upgrade: websocket" \
--header "Host: echo.websocket.org" \
--header "Origin: http://echo.websocket.org" \
--header "Sec-WebSocket-Version: 13" \
https://127.0.0.1:12345
```
