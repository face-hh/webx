# API Reference

How to work around with WebX's API, hosted at https://api.buss.lol.

This is the URI of the WebX API, which holds all the DNS of the network. You got different endpoints to do your stuff. https://api.buss.lol/.

{% hint style="warning" %}
APIs have rate limits. They are provided in the headers.
{% endhint %}

## `GET` /

_Provides a basic message explaining the API._

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL            |
| -------------- | --------------------- |
| `GET`          | https://api.buss.lol/ |

{% hint style="success" %}
#### RETURNS

```txt
Hello, world! The available endpoints are:
GET /domains,
GET /domain/:name/:tld,
POST /domain,
PUT /domain/:key,
DELETE /domain/:key,
GET /tlds.
Ratelimits provided in headers.
```
{% endhint %}

## `GET` /domains`?amount={x}&page={y}`

Being `amount` and `page` optional. `amount` for the amount of domains you want the response to have per page (defaults to 15), and `page` what page you want to view. _Allows you to get the list of all working domains from the network._

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                   |
| -------------- | ---------------------------- |
| `GET`          | https://api.buss.lol/domains |

{% hint style="success" %}
#### RETURNS

{% code title="response.json" overflow="wrap" lineNumbers="true" %}
```json
{
    "domains": [
        {
            "tld": "it",
            "name": "register",
            "ip": "https://github.com/face-hh/webx-registrar"
        },
        {
            "tld": "it",
            "name": "dingle",
            "ip": "https://github.com/face-hh/dingle-frontend"
        }
    ],
    ...
    "page": 1,
    "limit": 15
}
```
{% endcode %}
{% endhint %}

## `GET` /tlds

_Allows you to get the list of all valid TLDS._

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                |
| -------------- | ------------------------- |
| `GET`          | https://api.buss.lol/tlds |

{% hint style="success" %}
#### RETURNS

`200 OK`

```json
["mf", "btw", "fr", "yap", "dev", "scam", "zip", "root", "web", "rizz", "habibi", "sigma", "now", "it", "soy", "lol", "uwu"]
```
{% endhint %}

## `GET` /domain/`name`/`tld`

_Allows you to get the data from a specific domain._\
_**Being**** ****`name`**** ****the domain name (e.g. "register") and**** ****`tld`**** ****it's TLD (e.g. "it").**_

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                           |
| -------------- | ------------------------------------ |
| `GET`          | https://api.buss.lol/domain/name/tld |

{% hint style="success" %}
#### RETURNS

#### IF DOMAIN DOES EXIST

```json
{
    "tld":"it",
    "name":"register",
    "ip":"https://github.com/face-hh/webx-registrar"
}
```
{% endhint %}

{% hint style="danger" %}
#### IF DOMAIN DOES NOT EXIST

_Does not return anything._
{% endhint %}

## `POST` /domain

_Allows you to register a domain from the API_

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                  | HEADERS                          |
| -------------- | --------------------------- | -------------------------------- |
| `POST`         | https://api.buss.lol/domain | `Content-Type: application/json` |

{% hint style="info" %}
_**AND BODY:**_

```json
{
    "tld": "{tld}",
    "name": "{name}",
    "ip": "{ip}"
}
```

_Being `{name}` the `name` you want to use as the domain, `{tld}` the TLD you want to use, and `{ip}` the IP / GitHub URL you want to serve from._
{% endhint %}

{% hint style="success" %}
#### RETURNS

#### IF THE DOMAIN IS CREATED

`200 OK`

```json
{
    "tld": "example_tld",
    "ip": "example_ip",
    "name": "example_name",
    "secret_key": "generated_secret_key"
}
```
{% endhint %}

{% hint style="danger" %}
#### IF THE BODY OF YOUR `POST` REQUEST IS NOT VALID

`400 Bad Request`

#### IF `{name}` (domain name) IS ALREADY REGISTERED WITH THAT TLD

`409 Bad Request`

#### IF RATE LIMIT HAS BEEN EXCEEDED

`429 Too Many Requests`
{% endhint %}

## `POST` /domain/check

_Allows you to "search" for domains using domain names and TLDs_

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                        | HEADERS |
| -------------- | --------------------------------- | ------- |
| `POST`         | https://api.buss.lol/domain/check | _None_  |

{% hint style="info" %}
_**AND BODY:**_

```json
{
    "tld": "{tld}",
    "name": "{name}"
}
```

_Being `{name}` the `name` you want to search for, and `{tld}` an **optional** parameter to also search for a specific TLD._
{% endhint %}

{% hint style="info" %}
Quick reminder: unless specified, all parameters are required.
{% endhint %}

{% hint style="success" %}
#### RETURNS

#### IF THE DOMAIN IS CREATED

`200 OK`

```json
{
    "tld": "example_tld",
    "ip": "example_ip",
    "name": "example_name",
    "secret_key": "generated_secret_key"
}
```
{% endhint %}

{% hint style="danger" %}
#### IF THE BODY OF YOUR `POST` REQUEST IS NOT VALID

`400 Bad Request`

#### IF `{name}` (domain name) IS ALREADY REGISTERED WITH THAT TLD

`409 Bad Request`

#### IF RATE LIMIT HAS BEEN EXCEEDED

`429 Too Many Requests`
{% endhint %}

## `PUT` /domain/`key`

_Allows you to update your domain's IP / GitHub URL. The code's source, basically._

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                       | HEADERS                          |
| -------------- | -------------------------------- | -------------------------------- |
| `PUT`          | https://api.buss.lol/domain/:key | `Content-Type: application/json` |

{% hint style="info" %}
_Being `:key` your domain's secret key._

_**AND BODY:**_

```json
{
    "ip": "{ip}"
}
```

_Being `{ip}` the new IP you want to set for your domain._
{% endhint %}

{% hint style="success" %}
#### RETURNS

#### IF THE IP IS CORRECTLY UPDATED

`200 OK`

{% code title="response.json" overflow="wrap" lineNumbers="true" %}
```json
{
    "ip": "new_ip"
}
```
{% endcode %}
{% endhint %}

{% hint style="danger" %}
#### IF THE BODY OF YOUR `PUT` REQUEST IS NOT VALID _OR_ SPECIFIED `KEY` IS NOT VALID

`400 Bad Request`

#### IF THE DOMAIN IS NOT FOUND

`404 Bad Request`
{% endhint %}

## `DELETE` /domain/`key`

_Allows you to delete your domain from the network. You cannot undo that, so be careful._

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                       | HEADERS               |
| -------------- | -------------------------------- | --------------------- |
| `DELETE`       | https://api.buss.lol/domain/:key | _No headers required_ |

{% hint style="info" %}
_Being `:key` your domain's secret key._
{% endhint %}

{% hint style="success" %}
#### RETURNS

#### IF THE DOMAIN IS CORRECTLY REMOVED

`200 OK`
{% endhint %}

{% hint style="danger" %}
#### IF THE REQUEST HAS AN INVALID PARAMETER

`400 Bad Request`

#### IF THE DOMAIN IS NOT FOUND

`404 Bad Request`
{% endhint %}

## `POST` /registry/domain

_Allows to create your own domain using an API key._

{% hint style="warning" %}
This is disabled by default as you will need to come up with your own way of validating and distributing API Keys.
{% endhint %}

{% hint style="info" %}
#### YOU SEND
{% endhint %}

| REQUEST METHOD | TARGET URL                            | HEADERS                                                      |
| -------------- | ------------------------------------- | ------------------------------------------------------------ |
| `POST`         | https://api.buss.lol/registry/domain/ | `Content-Type: application/json; Authorization = name:token` |

{% hint style="info" %}
_Being `:token` your API key._

_**AND BODY:**_

```json
{
    "tld": "{tld}",
    "name": "{name}",
    "ip": "{ip}"
}
```

_Being `{name}` the `name` you want to use as the domain, `{tld}` the TLD you want to use, and `{ip}` the IP / GitHub URL you want to serve from._
{% endhint %}

{% hint style="success" %}
#### RETURNS

#### IF THE DOMAIN IS SUCCESSFULLY CREATED

`200 OK`

```json
{
    "tld": "example_tld",
    "ip": "example_ip",
    "name": "example_name",
    "secret_key": "generated_secret_key"
}
```
{% endhint %}

{% hint style="danger" %}
#### IF THE BODY OF YOUR `POST` REQUEST IS NOT VALID

`400 Bad Request`

#### IF THE API KEY SYSTEM IS DISABLED

(It can be.)

`403 Not allowed`

#### IF THE REQUEST DOMAIN IS ALREADY REGISTERED

`409 Conflict`

#### IF THE RATE LIMIT IS EXCEEDED

`429 Too many requests`
{% endhint %}
