# Tracking-App
This is the repo to hold the tracking apps backend. This is a joint effort with DylanPatel401.


##End-point

https://tracking-app-docker.onrender.com

###Get Requests

```
- /records/records/<username>/<passwordhash>
- /records/users/<username>/<passwordhash>
```

###Post Requests

```
- /records/user/create    -d '{"id": 0, "username" : "John", "password_hash" : "sdapoios"}'
```
