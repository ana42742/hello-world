# Login validation API written in Rust

This project uses the `lambda_flows` and `flowsnet_platform_sdk` libraries to create a serverless function that handles user login requests. It validates JSON login data, compares it against predefined credentials, and responds with success or failure messages.

If valid, it returns "Login successful for user: username," else "Login failed: Invalid username or password."

Here is the [Lambda endpoint](https://code.flows.network/lambda/7V25JR7yqq) for this API.

Example Usage:

Go to your Terminal, ensure that you have `curl` installed and enter the following:

```
curl -X POST "https://code.flows.network/lambda/7V25JR7yqq" -H "Content-Type: application/json" -d '{"username":"test_user","password":"test_pass"}'
```

Output:
`Login failed: Invalid username or password`

```
curl -X POST "https://code.flows.network/lambda/7V25JR7yqq" -H "Content-Type: application/json" -d '{"username":"test_user","password":"test_pass"}'
```

Output:
`Login successful for user: abc`
