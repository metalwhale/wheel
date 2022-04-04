# Infrastructure for development

## Starting up
Create `.env` file and specify variables:
```
host$ cp .env.example .env
```
Start and get into the containers:
<pre>
host$ docker compose up -d
host$ docker compose exec <b>service</b> bash
</pre>
