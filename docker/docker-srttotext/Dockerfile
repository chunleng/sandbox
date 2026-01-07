FROM alpine:3.23.2

RUN apk add --no-cache bash git && \
    git clone https://github.com/oshenc/srttotext.git /app && \
    cd app && \
    git checkout 3c0d74990e24ed5afa9827fff4a6f2a2c88f0886 && \
    chmod +x /app/srt2text.sh && \
    ln -s /app/srt2text.sh /usr/local/bin/srt2text

WORKDIR /data

ENTRYPOINT ["srt2text"]
