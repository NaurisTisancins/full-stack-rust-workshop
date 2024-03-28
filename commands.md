# start local shuttle server iin watch mode, so that the code is recompiled after save

## cargo watch -x "shuttle run"

# deploy changes to shuttle

## cargo shuttle deploy

psql -h localhost -p 21341 -U postgres -d postgres
