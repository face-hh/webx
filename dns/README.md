# Domain Management API

This is a Domain Management API built with Actix Web and MongoDB. It provides endpoints to create, read, update, and delete domain information, along with rate limiting for certain operations.

## Table of Contents

- [Endpoints](#endpoints)
  - [GET /](#get-)
  - [POST /domain](#post-domain)
  - [GET /domain/:name/:tld](#get-domainnametld)
  - [PUT /domain/:key](#put-domainkey)
  - [DELETE /domain/:key](#delete-domainkey)
  - [GET /domains](#get-domains)
  - [GET /tlds](#get-tlds)

## Endpoints

### GET /

Returns a simple message with the available endpoints and rate limits.

**Response:**

```
Hello, world! The available endpoints are:
GET /domains,
GET /domain/{name}/{tld},
POST /domain,
PUT /domain/{key},
DELETE /domain/{key},
GET /tlds.
Ratelimits are as follows: 10 requests per 60s.
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
- `400 Bad Request` if the request body is invalid, the TLD is non-existent, the name is too long (24 chars), or the domain is offensive.
- `409 Conflict` if the domain already exists.

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
- `404 Not Found` if the domain is not found.

### DELETE /domain/:key

Deletes a domain entry using its secret key.

**Request:**

- Method: `DELETE`
- URL: `/domain/:key`
- Parameters:
  - `key`: The secret key of the domain.

**Response:**

- `200 OK` if the domain is successfully deleted.
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

### GET /tlds

Fetches the list of allowed top-level domains.

**Request:**

- Method: `GET`
- URL: `/tlds`

**Response:**

- `200 OK` with a list of TLDs.
  ```json
  ["example_tld1", "example_tld2", ...]
  ```

---

This README provides an overview of the API's endpoints and their expected behavior based on the provided code. Please note that the actual list of allowed TLDs and offensive words are loaded from the application's configuration.
