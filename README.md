### INSTALLATION:
`https://dgraph.io/downloads` -- to Install GraphQL (I used the linux installation, there is a startup script for that in a file called startup.sh, but the Docker installation should work fine)

### Files:
`curl.txt` -- Curl example that I had created from trial and error. This really should have been in their documentation. Hopefully this will help you out.
`http_api.rs` -- Started making some http calls.
`main.rs` -- For running the API calls. *.schema -- Target files for HTTP calls (look at .txt files for example of use). Do not think this will be the final way to do things but it works for now.

### Documentation:
`https://dgraph.io/docs/graphql/` -- General Dgraph 
`https://dgraph.io/docs/clients/raw-http/` -- HTTP Dgraph
`https://docs.rs/http_req/0.7.2/http_req/index.html` -- Rust Http library
