# Express Typescript Starter helper cli

### Installation

* Clone the repo
* ```cd ets-cli && cargo install```

### Usage
* ets [g|generate] [to_generate] [file_name]

### Example
` ets g c auth `

**This will create a file in src/controller/auth.controller.ts**


## Arguments

### _First Argument_
>  **g or generate**



### _Second Argument_
|arg|result|
|---|---|
|c or controller| creates a new controller |
|s or service | creates a new service |
|m or model | creates a new model |
|mw or middleware | creates a new middleware |
|rs or request-schema | create a new express validator schema|
|r or route | creates a new route |   
