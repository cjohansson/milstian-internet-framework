# NGINX

This document will describe various parts of integration with a nginx web-server.

## Use milstian via virtual host proxy

Add a location logic similar to this inside a `server` block:

```nginx
    location / {
        proxy_pass  http://localhost:8888;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
```

Replace port and host with your settings used when starting milstian.



[Back to start](../../../)
