FROM ubuntu:25.04

RUN echo '#!/bin/bash\nwhile true; do $@; echo; sleep 3; done' > entrypoint.sh && chmod +x entrypoint.sh

ENTRYPOINT [ "./entrypoint.sh" ]
CMD [ "echo", "test" ]
