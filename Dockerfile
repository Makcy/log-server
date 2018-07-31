FROM alpine:latest

COPY ./ /app
WORKDIR /app

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories \
	&& mkdir $HOME/.cargo \
	&& mv config $HOME/.cargo/ \
	&& apk add --no-cache libgcc \ 
	&& apk add --no-cache --virtual .build-rust rust cargo \
	&& cargo build --release \
	&& cp target/release/log-server .\
	&& rm -rf target/ ~/.cargo/ \
	&& apk del --purge .build-rust \
	&& mkdir -p /data/logServer/

EXPOSE 3000

ENTRYPOINT ["./log-server"]


