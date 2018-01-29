FROM nginx:stable-alpine

WORKDIR /usr/share/nginx/html

COPY web/build/ /usr/share/nginx/html