FROM willsquire/diesel-cli

ENV DB_ADAPTER=
ENV DB_USER=
ENV DB_PASSWORD=
ENV DB_PATH=

RUN echo 'diesel --database-url="${DB_ADAPTER}://${DB_USER}:${DB_PASSWORD}@${DB_PATH}" $@' > ./entrypoint.sh \
    && chmod +x ./entrypoint.sh

ENTRYPOINT [ "sh", "./entrypoint.sh"  ]
CMD [ "migration", "run" ]
