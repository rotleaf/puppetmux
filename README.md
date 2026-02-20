# puppetmux
_api to control tmux (not there yet)_

## Sessions
### GET /session/new/<session_name>
create a new session

- session_name is optional

#### Response
```json
{
  "message":"session mysession created!",
  "success": true
}
```

### GET /session/list
list all active sessions 

#### Response
```json
{
  "success": true,
  "sessions": [
    { "name": "mysession", "windows": "2", "created": "1771502202" },
    { "name": "mysession2", "windows": "2", "created": "1771502202" }
  ]
}
```

### GET /session/kill/<session_name>
kill a session by name

- session_name is required

#### Response
```json
{
  "message":"session mysession killed!", 
  "success": true
}
```

## windows
### GET /window/new/<session_name><window_name>
create a new window 

- session_name is required 
- window_name is optional

#### Response
```json
{
  "message":"window session_namr:window_name created!",
  "success":true
}
```

### GET /window/list/<session_name>
list windows in a session

- session_name is required 

#### Response 
```json
{
  "success": true,
  "windows": [
     {
       "active":"0",
       "index":"1",
       "name":"win_2"
     },
     {
       "active":"0",
       "index":"3",
       "name":"win_4"
     }
  ]
}
```

### GET /window/kill/<session_name>:<window_id>
kill a window

#### Response
```json
{
  "message":"window session:2 killed!",
  "success": true
}
```

### GET /window/split/session_name:window_id/orientation

- session_name, window_id & orientation required

#### Response
```
{
  "message":"window session_name:window_id split orientation!",
  "success":true
}
```