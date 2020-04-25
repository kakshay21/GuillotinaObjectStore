guillotina_grpc
===============

# Setup
```
pip install -r requirements.txt
python setup.py install
guillotina serve
```
# Check `@foobar` endpoint
```
curl -i -X POST http://localhost:8080/@foobar --user root:root
```

# More on Guillotina

```
curl -i -X POST http://localhost:8080/db/ -H 'Accept: application/json' -H 'Content-Type: application/json' --data-raw '{"@type": "Container", "description": "Description", "id": "container", "title": "Container 1"}' --user root:root
```
* Sample Request
```
curl --user root:root http://localhost:8080/db/container
// response
//{"@id": "http://localhost:8080/db/container", "@type": "Container", "@name": "container", "@uid": "7f0c0fc706234596b1f6db52a8535835", "@static_behaviors": [], "parent": {}, "is_folderish": true, "creation_date": "2020-04-25T21:49:58.521013+00:00", "modification_date": "2020-04-25T21:49:58.521013+00:00", "type_name": "Container", "title": "container", "uuid": "7f0c0fc706234596b1f6db52a8535835", "__behaviors__": [], "items": [], "length": 0}
```
* Sample POST request
```
curl -i -X POST http://localhost:8080/db/container -H 'Accept: application/json' --data-raw '{ "@type": "Item", "id": "foobar5" }' --user root:root
```
* Response
```
HTTP/1.1 201 Created
Access-Control-Allow-Credentials: true
Access-Control-Expose-Headers: *
Location: http://localhost:8080/db/container/foobar5
Content-Type: application/json
Server: Guillotina/5.3.37
Content-Length: 138
Date: Sat, 25 Apr 2020 21:59:05 GMT

{"@id": "http://localhost:8080/db/container/foobar5", "@name": "foobar5", "@type": "Item", "@uid": "7f0|e764269125474fb5b7e1d8e0d4d730c5"
```
* Sample GET request
```
curl --user root:root http://localhost:8080/db/container/foobar5
```
* Response
```
{"@id": "http://localhost:8080/db/container/foobar5", "@type": "Item", "@name": "foobar5", "@uid": "7f0|e764269125474fb5b7e1d8e0d4d730c5", "@static_behaviors": ["guillotina.behaviors.dublincore.IDublinCore"], "parent": {"@id": "http://localhost:8080/db/container", "@name": "container", "@type": "Container", "@uid": "7f0c0fc706234596b1f6db52a8535835"}, "is_folderish": false, "creation_date": "2020-04-25T21:59:05.456815+00:00", "modification_date": "2020-04-25T21:59:05.456815+00:00", "type_name": "Item", "title": null, "uuid": "7f0|e764269125474fb5b7e1d8e0d4d730c5", "__behaviors__": [], "guillotina.behaviors.dublincore.IDublinCore": {"title": null, "description": null, "creation_date": "2020-04-25T21:59:05.456815+00:00", "modification_date": "2020-04-25T21:59:05.456815+00:00", "effective_date": null, "expiration_date": null, "creators": ["root"], "tags": null, "publisher": null, "contributors": ["root"]}}
```