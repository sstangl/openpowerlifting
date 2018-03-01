# Running in Docker

#### Prerequisites
1) You have successfully executed `make`
2) Docker is installed and running
3) You are in the project root dir

#### The Meat
```
docker build -t openpl:${SOME_TAG} .
docker run -d -p ${SOME_PORT}:80 openpl:latest
```

In your browser, navigate to `localhost:${SOME_PORT}`

Enjoy your newly Dockerised application! 

## Thoughts?
Seems to load much quicker than simply loading the HTML files directly in your browser... 

## Improvements?
It's not a big job to build + test + Dockerise app manually as it is, but would be nice to automate it. Otherwise might get tedious if you're doing it often!