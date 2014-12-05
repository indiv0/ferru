Title: Snippets
Date: 2014-03-02
Slug: snippets
Author: Nikita Pekin
Summary: A collection of various useful snippets from my projects

This is a collection of various snippets I've written or found which I use in my projects.
In here you'll find basically anything I've used once but haven't actually included in a project.

## Timing

Initially from [here](http://stackoverflow.com/questions/5478351/python-time-measure-function).

Use this snippet to time the duration of any function by adding an `@timing` annotation to a function.

```python
import time


def timing(f):
    def wrap(*args):
        time1 = time.time()
        ret = f(*args)
        time2 = time.time()
        print('{} function took {} ms'.format(f.func_name,
                                              (time2-time1)*1000.0))
        return ret
    return wrap
```

## Minecraft Server Info

Initially from [here](http://pastebin.com/ZavsWG60).

```python
import jsonify
import socket


def serverping(host, port):
    try:
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.connect((host, port))
        #Send 0xFE: Server list ping
        s.send('\xfe\x01\xFA')

        #Read some data
        d = s.recv(1024)
        s.close()

        #Check we've got a 0xFF Disconnect
        assert d[0] == '\xff'

        #Remove the packet ident (0xFF) and the short containing the length of the string
        #Decode UCS-2 string
        d = d[3:].decode('utf-16be')

        #Check the first 3 characters of the string are what we expect
        assert d[:3] == u'\xa7\x31\x00'

        #Split
        d = d[3:].split('\x00')

        #Return a dict of values
        return jsonify(protocol_version=int(d[0]),
                       server_version=d[1],
                       motd=d[2],
                       players=int(d[3]),
                       max_players=int(d[4]))
    except:
        return jsonify(success=False)
```

## Minecraft Server Status ##

Most of the HTML/CSS/Python in this snippet is from xPaw's [mcstatus](http://xpaw.ru/mcstatus/).

### Python ###

```python
import json
import requests

@app.route('/hooks/mcstatus')
def mcstatus():
    data = {'success': True}
    try:
        r = requests.get('http://xpaw.ru/mcstatus/status.json')
        data.update(json.loads(r.text))
    except:
        data['success'] = False
    return json.dumps(data)
```

### HTML ###

```html
<div class="text-center">
  <div id="login" class="span2">
    <div class="service">
      <div class="name">
        Login
      </div>
      <h2 class="status">
      </h2>
    </div>
  </div>
  <div id="session" class="span2">
    <div class="service">
      <div class="name">
        Session
      </div>
      <h2 class="status">
      </h2>
    </div>
  </div>
  <div id="website" class="span2">
    <div class="service">
      <div class="name">
        Website
      </div>
      <h2 class="status">
      </h2>
    </div>
  </div>
  <div id="skins" class="span2">
    <div class="service">
      <div class="name">
        Skins
      </div>
      <h2 class="status">
      </h2>
    </div>
  </div>
  <div id="realms" class="span2">
    <div class="service">
      <div class="name">
        Realms
      </div>
      <h2 class="status">
      </h2>
    </div>
  </div>
</div>
```

### CSS ###

```css
.text-center {
  font-size: 1em;
  line-height: 1.4;
  font-family: 'Lato',Helvetica,Arial,sans-serif;
  color: rgb(255, 255, 255);
  text-align: center;
}
.span2 {
  margin-bottom: 15px;
  width: 140px;
  margin: 0px 1em;
  display: inline-block;
}
.service {
  width: 140px;
  height: 140px;
  padding: 1.2em 0px;
  margin: 0px 0px 0.5em;
  border: 3px solid rgb(0, 175, 237);
  border-radius: 50%;
  color: rgb(0, 175, 237);
  font-size: 1em;
  line-height: 1.4;
  font-family: 'Lato',Helvetica,Arial,sans-serif;
}
.up .service {
  border-color: rgb(145, 255, 0);
  color: rgb(145, 255, 0);
}
.down .service {
  border-color: rgb(255, 0, 21);
  color: rgb(255, 0, 21);
}
.service .status {
  font-weight: 300;
}
.service h2 {
  line-height: 1.4;
  font-family: 'Lato',Helvetica,Arial,sans-serif;
  font-size: 1.5em;
  margin: 0.83em 0px;
}
```

### JS ###

```js
$.getJSON('/hooks/mcstatus', function(data){
  console.log(data['report']);
  if(data.success == true) {
    $.each( data['report'], function( key, val ) {
      var $node = $('#' + key);
      $node.addClass(val['status']);
      $node.find('h2').append(val['title']);
    });
  }
});
```
