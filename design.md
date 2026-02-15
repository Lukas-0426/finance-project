## basic design for finance app



```mermaid
sequenceDiagram
    participant calander
    participant rest-api
    participant backend
    calander->>rest-api:user updates data field
    rest-api->>backend:send over wire

    loop Healthcheck 
    backend->>backend: some health check, maybe sse?
    end

    backend->>backend:update db after calculation
    backend->>rest-api:send results back 
    rest-api->>calander: display results in gui
 ```