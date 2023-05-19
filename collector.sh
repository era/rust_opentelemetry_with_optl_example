docker run  \
    -p 4317:4317 -p 13133:13133 \
    -v $(pwd)/otel-collector-config.yaml:/etc/otelcol/config.yaml \
    -v $(pwd)/metrics.json:/etc/otelcol/metrics.json \
    otel/opentelemetry-collector:latest
