# Rust API

## Table of contents
* [Authentication](#authentication)
* [Endpoints](#endpoints)

## Authentication
This API uses **PASETO v4** tokens for secure authentication. The authentication process is structured as follows:

1. **Login and Cookie Assignment**:     
    When a user logs in, the API creates a PASETO token with the following claims:

    * A **session UUID**, unique to this login session.
    * If the user selected "Remember Me", their **role** and their **user ID** from the database is also added to the token. This token is then stored as a cookie on the client.

    This token is referred to as the session's "**public**" token.

2. **Session Storage in Redis**:    
    In Redis, a key-value pair is created to store the session data:

    * The **key** is the session UUID prefixed to distinguish it as a session key.
    * The **value** is a second PASETO token that securely holds all necessary session data.

    This Redis token is referred to as the session's "**data**" token.
3. **Session Verification**:
    * When a user makes an authenticated request, the API decrypts the session's **public** token from the cookie and extracts the session UUID.
    * Using this session UUID, the API attempts to retrieve the corresponding session data token from Redis.
        * **If the Redis session's data token exists:** The token is decrypted, and session data is extracted, allowing verification to succeed.
        * **If the Redis session token is missing:**
            * The API checks if the session can be "renewed" by querying the database using the user ID from the public token.
            * If the user ID is available and the renewal conditions are met, a new session is created in Redis, and verification succeeds.
            * If the user ID is unavailable (e.g., "Remember Me" was not selected) or renewal fails, verification fails.

This approach ensures proper handling of session expiry and allows for secure and flexible session data storage using Redis.

## Endpoints
**NOTE:** Endpoints that have a "Role" description require specific user roles to access. If a user without the required role(s) attempts to access one of these endpoints, or the session cookie is not sent, the following responses will be returned:
* User does not have the required role: `HTTP 401`
```
{
    error: "You do not have the role required to access this endpoint.",
    roleRequired: "client" | "employee" | "admin"
}
```
* Session expired or session is invalid: `HTTP 401`
```
{
    error: "Failed to verify session: Error"
}
```
* Did not receive the "session" cookie: `HTTP 400`
```
{
    error: "Session cookie is missing."
}
```


### Health Check
---
* **URL**: `/health-check`
* **Method**: `GET`
* **Description**: Can be used for checking if the server is up.
* **Response**:
    * Success: `HTTP 200`
    ```
    {
        "Application is safe and healthy. :)"
    }
    ```
    * Unknown error: Literally hangs.

### User Registration
---
* **URL**: `/users/register`
* **Method**: `POST`
* **Description**: Registers a new user and sends a user verification email.
* **Request Body**:
```
{
    "email": "napstablook@undernet.com",
    "password": "12345678",
    "name": "Napstablook",
    "username": "NAPSTABLOOK22",
    "client"?: {
        age: 21,
        gender: "other",
        phoneNum: "999-9999-999",
        interests: ["clothes", "tech", "library", "food"]
    },
    "employee"?: {
        age: 21,
        gender: "other",
        phoneNum: "999-9999-999",
        schedule: [{
            enterDate: DateTimeUtc,
            exitDate: DateTimeUtc,
            store: ObjectId,
            storeJob: ObjectId
        }]
    },
    "admin"?: {}
}

NOTE: Must contain either client, employee, or admin.
```
* **Response**:
    * Success: `HTTP 200`
    ```
    {
        message: "Verification email sent."
    }
    ```
    * Attempted to create a user with an email/username that already exists: `HTTP 409`
    * Unknown error: `HTTP 500`

### User Email Verification
---
* **URL**: `/users/register/verify`
* **Method**: `PUT`
* **Description**: Decrypts the email confirmation token and activates a registered user on the database.
* **Parameters**:
    * `token`: PASETOv4 email confirmation token.
* **Response**:
    * Success: `HTTP 200`
    ```
    {
        message: "User activated successfully."
    }
    ```
    * Token expired/cannot verify account: `HTTP 303`

### User Login
---
* **URL**: `/users/login`
* **Method**: `POST`
* **Description**: Logs in a user and issues a session token pair.
* **Request Body**:
```
{
    email: "napstablook@undernet.com",
    password: "12345678",
    rememberMe: true
}
```
* **Response**:
    * Success (When user has only one role available): `HTTP 200` `Set cookie: (session: Session's public token)`
    ```
    {
        email: "napstablook@undernet.com",
        name: "Napstablook",
        role: "client",
    }
    ```
    * Success (When user has multiple roles available): `HTTP 200`
    ```
    {
        availableRoles: ["client", "employee", "admin"]
        token: PASETOv4 role selection public token.
    }
    ```
    * Already logged in with a valid session token: `HTTP 200`
    ```
    {
        message: "You are already logged in."
    }
    ```
    * User not found or invalid password: `HTTP 404`
    ```
    {
        error: String
    }
    ```
    * Unknown error: `HTTP 500`

### User Login Role Selection
---
* **URL**: `/users/role-login`
* **Method**: `POST`
* **Description**: Should be accessed after a login that resulted in multiple roles available. When this happens, the login endpoint will issue a role selection public token that must be passed to this endpoint to complete the login.
* **Request Body**:
```
{
    role: "client" | "employee" | "admin",
    token: PASETOv4 role selection public token.
}
```
* **Response**:
    * Success: `HTTP 200` (Sets the session cookie just like the `login` endpoint)
    ```
    {
        email: "napstablook@undernet.com",
        name: "Napstablook",
        role: "client",
    }
    ```
    * User does not have the role selected: `HTTP 400`
    ```
    {
        error: "User lacks {ROLE} role data."
    }
    ```
    * Unknown error: `HTTP 500`

### User Session Verification
---
* **URL**: `/users/verify-session`
* **Method**: `GET`
* **Description**: Verifies a user's session token.
* **Response**:
    * Success: `HTTP 200`
    * Session token cookie not present: `HTTP 400`
    * Verification failed or session expired: `HTTP 401`

### User Logout
---
* **URL**: `/users/logout`
* **Method**: `DELETE`
* **Description**: Logs out a user, revoking the UUID token cookie and the session data token from Redis.
* **Response**:
    * Success: `HTTP 200` `Clear cookie: session`
    * Session public token cookie not present: `HTTP 400`
    * Unknown error: `HTTP 500`

### Item Search
---
* **URL**: `/search`
* **Method**: `GET`
* **Description**: Returns the results for an item search query.
* **Parameters**:
    * `input`: Search query.
    * `page`: Number of "search" page.
    * `min-price`: Minimum item price.
    * `max-price`: Maximum item price.
* **Response**:
    * Found search results: `HTTP 200`
    ```
    [{
        id: ObjectId,
        name: "Intel A770 GPU",
        price: 499.99,
        store: "cyberion",
        coll: "techGpu"
    }]
    ```
    * Didn't find any result: `HTTP 200`
    ```
    []
    ```
      
### Item Search Suggestions
---
* **URL**: `/search-suggestions`
* **Method**: `GET`
* **Description**: Returns the suggestions for an item search input, for text autocompletion purposes. E.g., when the input is "int", this endpoint will lookup the items, and if it finds one called "Intel A770 GPU", it might return it as a suggestion, since its name contains "int".
* **Parameters**:
    * `input`: Search input.
* **Response**:
    * Found suggestions: `HTTP 200`
    ```
    [{
        name: "Intel A770 GPU",
        coll: "techGpu"
    }]
    ```
    * Didn't find any result: `HTTP 200`
    ```
    []
    ```

### Get Client's Cart Items
---
* **URL**: `/clients/cart`
* **Method**: `GET`
* **Description**: Returns the items in the client's cart.
* **Role**: `client`
* **Response**:
    * Success: `HTTP 200`
    ```
    [{
        id: ObjectId,
        name: "Intel A770 GPU",
        price: 499.99,
        store: "cyberion",
        inStock: true
    }]
    ```
    * Success (Cart is empty): `HTTP 200`
    ```
    []
    ```
    * Unknown error: `HTTP 500`
    
### Insert Item into Client's Cart
---
* **URL**: `/clients/cart`
* **Method**: `POST`
* **Description**: Inserts an item to the client's cart.
* **Role**: `client`
* **Parameters**:
    * `item`: ObjectId of the item to insert.
* **Response**:
    * Success: `HTTP 200`
    * Item already in the client's cart: `HTTP 400`
    ```
    {
        error: "The cart already has this item."
    }
    ```
    * Unknown error: `HTTP 500`

### Delete Item from Client's Cart
---
* **URL**: `/clients/cart`
* **Method**: `DELETE`
* **Description**: Delete an item to the client's cart.
* **Role**: `client`
* **Parameters**:
    * `item`: ObjectId of the item to delete.
* **Response**:
    * Success: `HTTP 200`
    * Item not in the client's cart: `HTTP 400`
    ```
    {
        error: "The item was not found in the cart."
    }
    ```