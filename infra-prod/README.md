# Infrastructure for production

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
