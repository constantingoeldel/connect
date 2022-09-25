consul agent -config-file=consul/config.json -bind '{{ GetInterfaceIP "tailscale0" }}'

