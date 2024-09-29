FROM ubuntu

RUN apt update
RUN apt install -y imagemagick-6.q16 librsvg2-bin
COPY convert.sh .
COPY logo.svg .
WORKDIR /host_dir/artwork/logo
CMD sh convert.sh
