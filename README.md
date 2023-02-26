# Getting the League Client Information
## macOS Example

If you want to make your own wrapper

```
--install-directory
`ps x -o args | grep 'LeagueClientUx'`
// $ ps -A | grep "RiotClientUx"
/Users/Shared/Riot\ Games/Riot\ Client.app/Contents/Frameworks/RiotClient.app/Contents/MacOS/RiotClientUx --app-port=53204 --remoting-auth-token=9LUrYWgptpGWrgGKfl-l9A --app-pid=3633 --log-dir=/Users/crobertson/Library/Logs/Riot\ Games/Riot\ Client --user-data-root=/Users/crobertson/Library/Application\ Support/Riot\ Games/Riot\ Client --app-root=/Users/Shared/Riot\ Games/Riot\ Client.app --crashpad-environment=KeystoneFoundationLiveMac

curl -u riot:LwyzYKy_w3CcoZL9QgT2zg --basic --insecure https://localhost:56970/swagger/v1/api-docs
curl /voice-chat/v1/config

curl --include \
-u riot:7_8wJLXlPciMsMEpMngACQ --basic --insecure \
--no-buffer \
--header "Connection: Upgrade" \
--header "Upgrade: websocket" \
--header "Host: echo.websocket.org" \
--header "Origin: http://echo.websocket.org" \
--header "Sec-WebSocket-Version: 13" \
https://127.0.0.1:53204
```
