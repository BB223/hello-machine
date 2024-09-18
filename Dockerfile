FROM scratch
COPY ./hello-machine ./
EXPOSE 3000
ENTRYPOINT [ "./hello-machine" ]

