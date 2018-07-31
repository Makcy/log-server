## Usage
**1、mkdir -p /data/logServer**

**2、cargo run**

curl -X POST -d {devops: 'what are u fucking doing!' } http://localhost:3000

Then, you can get a log file in /data/logServer like 2018-07-25-kong.log 

## Start With Docker
**1、docker build -t log-server .**
**2、docker run -d -p 3000:3000 -v /data/logServer/:/data/logServer/ alpine-server ./log-server**
