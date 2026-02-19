# puppetmux
_api to control tmux (trying...)_

## Sessions
### GET /session/new/:name
create a new session, name is optional

#### Response
```json
{"message":"session mysession created!"}
```

### GET /session/list
list all active sessions 

#### Response
```json
[
  { "name": "mysession", "windows": "2", "created": "1771502202" },
  { "name": "mysession2", "windows": "2", "created": "1771502202" }
]
```

### GET session/kill/:name
kill a session by name

#### Response
```json
{"message":"session mysession killed!"}
```