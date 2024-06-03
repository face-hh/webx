# Domain Management API

This folder includes the source code of the **API** for the **Bussin DNS**.

This docu created with chatGpt4o [ChatGpt Chat link](https://chatgpt.com/share/847ddf83-5fd9-42e7-bdac-fee8168d974c)

This is a simple Domain Management API built with Express.js and MongoDB. It provides endpoints to create, read, update, and delete domain information, along with rate limiting for certain operations.

## Table of Contents

- [Endpoints](#endpoints)
  - [GET /](#get-)
  - [POST /domain](#post-domain)
  - [POST /domainapi/:apiKey](#post-domainapiapikey)
  - [GET /domain/:name/:tld](#get-domainnametld)
  - [PUT /domain/:key](#put-domainkey)
  - [DELETE /domain/:id](#delete-domainid)
  - [GET /domains](#get-domains)
  - [GET /tlds](#get-tlds)


## Endpoints

### GET /

Returns a simple message with the available endpoints.

**Response:**

```
Hello, world! The available endpoints are:
GET /domains,
GET /domain/:name/:tld,
POST /domain,
PUT /domain/:key,
DELETE /domain/:key,
GET /tlds.
Ratelimits provided in headers.
```

### POST /domain

Creates a new domain entry.

**Request:**

- Method: `POST`
- URL: `/domain`
- Headers: 
  - `Content-Type: application/json`
- Body:
  ```json
  {
    "tld": "example_tld",
    "ip": "example_ip",
    "name": "example_name"
  }
  ```

**Response:**

- `200 OK` if the domain is successfully created.
  ```json
  {
    "tld": "example_tld",
    "ip": "example_ip",
    "name": "example_name",
    "secret_key": "generated_secret_key"
  }
  ```
- `400 Bad Request` if the request body is invalid.
- `409 Conflict` if the domain already exists.
- `429 Too Many Requests` if the rate limit is exceeded.

### POST /domainapi/:apiKey

Creates a new domain entry using an API Key. This is disabled by default as you will need to come up with your own way of validating and distributing API Keys.

**Request:**

- Method: `POST`
- URL: `/domainapi/:apiKey`
- Headers: 
  - `Content-Type: application/json`
- Body:
  ```json
  {
    "tld": "example_tld",
    "ip": "example_ip",
    "name": "example_name"
  }
  ```

**Response:**

- `200 OK` if the domain is successfully created.
  ```json
  {
    "tld": "example_tld",
    "ip": "example_ip",
    "name": "example_name",
    "secret_key": "generated_secret_key"
  }
  ```
- `400 Bad Request` if the request body is invalid.
- `403 Not allowed` if the API key system is disabled.
- `409 Conflict` if the domain already exists.
- `429 Too Many Requests` if the rate limit is exceeded.

### GET /domain/:name/:tld

Fetches a domain entry by name and TLD.

**Request:**

- Method: `GET`
- URL: `/domain/:name/:tld`
- Parameters:
  - `name`: The domain name.
  - `tld`: The top-level domain.

**Response:**

- `200 OK` if the domain is found.
  ```json
  {
    "tld": "example_tld",
    "name": "example_name",
    "ip": "example_ip"
  }
  ```
- `404 Not Found` if the domain is not found.

### PUT /domain/:key

Updates the IP address of a domain entry using its secret key.

**Request:**

- Method: `PUT`
- URL: `/domain/:key`
- Parameters:
  - `key`: The secret key of the domain.
- Headers:
  - `Content-Type: application/json`
- Body:
  ```json
  {
    "ip": "new_ip_address"
  }
  ```

**Response:**

- `200 OK` if the IP address is successfully updated.
  ```json
  {
    "ip": "new_ip_address"
  }
  ```
- `400 Bad Request` if the request body or key is invalid.
- `404 Not Found` if the domain is not found.

### DELETE /domain/:id

Deletes a domain entry using its secret key.

**Request:**

- Method: `DELETE`
- URL: `/domain/:id`
- Parameters:
  - `id`: The secret key of the domain.

**Response:**

- `200 OK` if the domain is successfully deleted.
- `400 Bad Request` if the request parameter is invalid.
- `404 Not Found` if the domain is not found.

### GET /domains

Fetches all domain entries.

**Request:**

- Method: `GET`
- URL: `/domains`

**Response:**

- `200 OK` with a list of domains.
  ```json
  [
    {
      "tld": "example_tld",
      "name": "example_name",
      "ip": "example_ip"
    },
    ...
  ]
  ```
- `500 Internal Server Error` if there is an error retrieving the domains.

### GET /tlds

Fetches the list of allowed top-level domains.

**Request:**

- Method: `GET`
- URL: `/tlds`

**Response:**

- `200 OK` with a list of TLDs.
  ```json
  ["mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu"]
  ```

---

This README provides a comprehensive overview of the API's endpoints and their expected behavior. Adjust the `<repository_url>` and `<repository_directory>` placeholders to match your actual repository details.
