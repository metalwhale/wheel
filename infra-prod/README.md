# Infrastructure for production

## Create `logger` app
Create config file and specify variables:
```
host$ cp logger/config.properties.example logger/config.properties
```
Build image:
<pre>
host$ docker build -t <i>wheel/logger:0.1.0</i> -f logger/Dockerfile ../logger/
</pre>
Deploying:
<pre>
host$ kubectl create configmap <i>logger-config</i> --from-env-file=logger/config.properties
host$ kubectl apply -f logger/deployment.yaml
</pre>

## Create `client` app
Create config file and specify variables:
```
host$ cp client/config.properties.example client/config.properties
```
Build image:
<pre>
host$ docker build -t <i>wheel/client:0.1.0</i> -f client/Dockerfile ../client/
</pre>
Deploying:
<pre>
host$ kubectl create configmap <i>client-config</i> --from-env-file=client/config.properties
host$ kubectl apply -f client/deployment.yaml
</pre>
