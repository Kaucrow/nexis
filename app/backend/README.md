# Rust API

## Table of contents
* [Authentication](#authentication)
* [Endpoints](#endpoints)

## Authentication
This API uses **PASETO v4** tokens for secure authentication. The authentication process is structured as follows:

1. **Login and Cookie Assignment**:     
    When a user logs in, the API creates a PASETO token with the following claims:

    * A **session UUID**, unique to this login session.
    * If the user selected "Remember Me", their **user ID** from the database is also added to the token. This token is then stored as a cookie on the client.

2. **Session Storage in Redis**:    
    In Redis, a key-value pair is created to store the session data:

    * The **key** is the session UUID prefixed to distinguish it as a session key.
    * The **value** is a second PASETO token that securely holds all necessary session data.

3. **Session Verification**:
    * When a user makes an authenticated request, the API decrypts the session UUID token from the cookie and extracts the session UUID.
    * Using this session UUID, the API attempts to retrieve the corresponding session data token from Redis.
        * **If the Redis session token exists:** The token is decrypted, and session data is extracted, allowing verification to succeed.
        * **If the Redis session token is missing:**
            * The API checks if the session can be "renewed" by querying the database using the user ID from the cookie token.
            * If the user ID is available and the renewal conditions are met, a new session is created in Redis, and verification succeeds.
            * If the user ID is unavailable (e.g., "Remember Me" was not selected) or renewal fails, verification fails.

This approach ensures proper handling of session expiry and allows for secure and flexible session data storage using Redis.

## Endpoints

### Health Check
---
* **URL**: `/health-check`
* **Method**: `GET`
* **Description**: Can be used for checking if the server is up
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
* **Method**: `GET`
* **Description**: Decrypts the email confirmation token and activates a registered user on the database.
* **Parameters**: `token: PASETOv4 email confirmation token`
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
* **Description**: Logs in a user and issues a session token.
* **Request Body**:
```
{
    email: "napstablook@undernet.com",
    password: "12345678",
    rememberMe: true
}
```
* **Response**:
    * Success: `HTTP 200` `Set cookie: (session_uuid, Session token)`
    ```
    {
        email: "napstablook@undernet.com",
        name: "Napstablook"
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
        message: String
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
* **Method**: `POST`
* **Description**: Logs out a user, revoking the UUID token cookie and the session data token from Redis.
* **Response**:
    * Success: `HTTP 200` `Clear cookie: session_uuid`
    * Session token cookie not present: `HTTP 400`
    * Unknown error: `HTTP 500`
