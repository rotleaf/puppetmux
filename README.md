# puppetmux
_api to control tmux (not there yet)_

## Auth
- set *API_KEY* environment variable to enable auth, the value of that environment variable becomes your api key.
- authenticate with an header `api-key: yourkeyhere`. All requests are authenticated in this case.

## Sessions
### GET /session/new/<session_name>
create a new session

- session_name is optional

#### Response
```json
{
  "message":"session <session_name> created!",
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
  "message":"session <session_name> killed!", 
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
  "message":"window <session_name>:<window_name> created!",
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
  "message":"window <session_name>:<window_id> killed!",
  "success": true
}
```

### GET /window/split/<session_name>:<window_id>/<orientation>
split a window, orientation can be horizontal or vertical

- session_name, window_id & orientation required

#### Response
```json
{
  "message":"window <session_name>:<window_id> split <orientation>!",
  "success":true
}
```

## Panes
### GET /pane/list/<session_name>:<window_id>
list panes in a window

- session_name & window_id are required

#### Response
```json
{
  "panes": [
    {
      "active": true,
      "command": "empty",
      "height": "20",
      "id": "%0",
      "index": "0",
      "last_program": "zsh",
      "pid": "1927",
      "width": "82"
    },
    {
      "active": false,
      "command": "/system/bin/ping -q google.com",
      "height": "19",
      "id": "%1",
      "index": "1",
      "last_program": "ping",
      "pid": "1935",
      "width": "82"
    }
  ],
  "success": true
}
```

### GET /pane/select/<session_name>:<window_id>.<pane_id>
select a pane to make it active

- session_name, window_id & pane_id are required

#### Response
```json
{
  "message": "pane <session_name>:<window_id>.<pane_id> selected!",
  "success": true
}
```

### GET /pane/kill/<session_name>:<window_id>.<pane_index>
kill a pane 

- session_name, window_id & pane_id are required 

#### Response
```json
{
  "message": "pane <session_name>:<window_id>.<pane_id> killed!",
  "success": true
}
```

### GET /pane/read/<session_name>:<window_id>.<pane_index>
read contents of a pane

- session_name, window_id & pane_id are required 
_returns unfiltered content_

#### Response 
```json
{
  "output": "whatever is in the shell",
  "success": true
}
```

## using pane ids (they look like %1)
- _responses don't differ_
- <pane_id> is required


### GET /pane/<pane_id>/read
- read contents of a pane.

### GET /pane/<pane_id>/kill
- kill a pane

### GET /pane/<pane_id>/select
- select a pane 

## Command Shortcuts
### GET /pane/<pane_id>/ctrl-c
- send ctrl + c to a pane
- pane_id is required

#### Response 
```json
{
  "message":"Ctrl+C sent to pane <pane_id>",
  "success":true
}
```

### GET /pane/<pane_id>/last-cmd
- get the last command to be ran on this pane (_like pressing arrow up_)
- pane_id is required

#### Response
```json
{
  "command":"curl localhost:3030/pane/%0/ctrl-c",
  "success":true
}
```

### GET /pane/<pane_id>/last-cmd/run
- rerun the previous command 
- pane_id is required

#### Response 
```json 
{
    "message":"last command rerun!",
    "success":true
}
```