# Tracking-App
This is the repo to hold the tracking apps backend. This is a joint effort with DylanPatel401.


## End-point

[https://tracking-app-docker.onrender.com](https://tracking-app-backend.onrender.com/)

### Get Requests

```
- /records/records/<username>/<passwordhash>
- /records/users/<username>/<passwordhash>
```

### Post Requests

```
- /records/user/create    -d '{"id": 0, "username" : "John", "password_hash" : "sdapoios"}'
```

### CURL requests 

#### Create user

```
curl -X POST https://tracking-app-backend.onrender.com/records/user/create -d '{"id": 0, "username" : "John", "password_hash" : "sdapoios"}'
```

#### Query for user

```
curl [http://localhost:8000](https://tracking-app-backend.onrender.com/)https://tracking-app-backend.onrender.com/records/users/John/sdapoios
```

#### Query for record

```
curl http://localhost:8000/records/records/<username>/<passwordhash>
```
