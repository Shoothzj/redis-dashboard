FROM shoothzj/base

COPY dist /opt/redis-dashboard

WORKDIR /opt/redis-dashboard

EXPOSE 10008

CMD ["/usr/bin/dumb-init", "/opt/redis-dashboard/redis-dashboard"]
