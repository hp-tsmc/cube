#! /bin/bash

docker run --rm -p 3000:3000 -p 4000:4000 \
-e CUBEJS_DB_HOST=postgres://localhost \
-e CUBEJS_DB_NAME=test \
-e CUBEJS_DB_USER=tset \
-e CUBEJS_DB_PASS=test \
-e CUBEJS_DB_TYPE=test \
-e CUBEJS_API_SECRET=test \
-e CUBEJS_DEV_MODE=true \
-v $(pwd):/cube/conf \
docker.io/library/cubejs:metrics