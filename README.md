# Rust Runtime for Lambda

This stack contains a Rust runtime for AWS Lambda that allows us to create
new Lambdas using only the template.yaml (similar to using built-in runtimes)

## Prerequisites

* GNU Make (version 4)
* SAM CLI

## How to run

```
sam build
sam local invoke TestRustLambda
sam local invoke AnotherLambda
```
