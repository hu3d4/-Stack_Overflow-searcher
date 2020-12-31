FROM nginx:alpine

#nginx.confをDockerの/etc/nginx/以下に配置
# COPY nginx.conf /etc/nginx/


#Alpine Linuxのアップデート
RUN apk update \
    && apk upgrade \
    && apk add --no-cache openssl \
    && apk add --no-cache bash

#www-dataを追加
RUN set -x ; \
    addgroup -g 82 -S www-data ; \
    adduser -u 82 -D -S -G www-data www-data && exit 0 ; exit 1

#ポートの80と443番を開放
EXPOSE 80 443