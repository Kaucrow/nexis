# Python API

## Table of contents
* [Authentication](#authentication)
* [Endpoints](#endpoints)

## Authentication
This API uses **PASETO v4** tokens for secure authentication. The authentication process is the same as the one described in the [Rust API documentation](../nexis-rs/README.md).

## Endpoints

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

### User Session Verification
---
* **URL**: `/users/verify-session`
* **Method**: `GET`
* **Description**: Verifies a user's session token.
* **Response**:
    * Success: `HTTP 200`
    * Session token cookie not present: `HTTP 400`
    * Verification failed or session expired: `HTTP 401`
    * Redis server down: `HTTP 500`
