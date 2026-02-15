## basic design for finance app

##### TODOS 
   make rpm/deb/portable to windows 
   set up rest endpoints for gui 
   read egui documentation 
   set up more of back end 

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



---
title: backend class logic 
---

 ```mermaid
    classDiagram
        note "how backend logic should flow" 
        note for account "total fields should be cummulation from vecs" 
        Finances :  vec income
        Finances :  vec expensies 
        Finances :  total_income
        Finances :  total_expensies 
        Calander :  month
        Calander :  weeks
        note for Calander "should allow user to add finances for one week or month, months are made of weeks" 
```