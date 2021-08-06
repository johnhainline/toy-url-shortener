# Description
This is a toy URL shortener application written in Rust (using Rocket).

This app uses Rust's `stable` build.
# To run
`cargo run`

## Testing (Happy Path)
I assume you have a new version of both `curl` and `jq` on the terminal.

### Create Shortened URL
```
SHORT=`curl -s --location --request POST 'http://localhost:8000/api/v1/create-shortened-url' --header 'Content-Type: application/json' --data-raw '{ "url": "https://www.google.com" }' | jq -r .short`
echo $SHORT
```

### Verify Shortened URL
`curl -s --location --request POST 'http://localhost:8000/api/v1/get-shortened-url' --header 'Content-Type: application/json' --data-raw '{ "url": "https://www.google.com" }'`

### Use Shortened URL (Expect Redirect)
`curl -s --location --request GET 'http://localhost:8000/'$SHORT`

## Testing (Sad Paths)
### Create Nonsense URL
`curl -s --location --request POST 'http://localhost:8000/api/v1/create-shortened-url' --header 'Content-Type: application/json' --data-raw '{ "url": "what://www.google.com" }'`

`curl -s --location --request POST 'http://localhost:8000/api/v1/create-shortened-url' --header 'Content-Type: application/json' --data-raw '{ "url": "http:/" }'`

### Verify Nonsense URLs
`curl -s --location --request POST 'http://localhost:8000/api/v1/get-shortened-url' --header 'Content-Type: application/json' --data-raw '{ "url": "https://www.never-added.com" }'`

### Use Nonsense URL
`curl -s --location --request GET 'http://localhost:8000/never-added'`

