# Criar tabela e subir ela

## Alteração do docker

1- Coloquei os docker dentro de uma pasta 

2- Alterei docker-compose

````
app:
    build:
      context: ..     # raiz do projeto
      dockerfile: docker/Dockerfile
    restart: always
    ports:
      - "8080:8080"   # ajuste para a porta da sua API
    environment:
      DATABASE_URL: postgres://user:password@db:5432/payment_db
    depends_on:
      - db
````

3 - Alterei o dockerfile

````
# instala libs de runtime comuns (ajuste conforme seu projeto)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
````

4- Criei init.sql 

-  Tabela do banco

5- Alterei o docker-compose para ele subir a tabela

````
 volumes:
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql
````