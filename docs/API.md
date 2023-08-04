# API

The API is a RESTful API that uses JSON for serialization and JWT for authentication.
CRUD operations are available for users, roles and permissions.

## Index

* [Authentication](#authentication)
    * [Register](#register)
    * [Login](#login)
    * [Current](#current)
* [Users](#users)
    * [Create](#create)
    * [Read](#read)
    * [Update](#update)
    * [Delete](#delete)
* [Roles](#roles)
    * [Create](#create-1)
    * [Read](#read-1)
    * [Update](#update-1)
    * [Delete](#delete-1)
* [Permissions](#permissions)
    * [Create](#create-2)
    * [Read](#read-2)
    * [Update](#update-2)
    * [Delete](#delete-2)

## Authentication

Authentication is handled using JSON Web Tokens (JWT). The following endpoints are available:

* `/authentication/register`
* `/authentication/login`
* `/authentication/current`

### Register

Registering an account will create a new account and provide it with the default role. Passwords will be hashed using
argon2 and a custom salt.

#### Request

```http
POST /authentication/register
{
  "username": "example",
  "email": "example@codedead.com",
  "firstName": "Jane",
  "lastName": "Doe",
  "password": "password"
}
```

#### Response

```http
200 OK
```

### Login

Logging in provides an access token that can be used to authenticate other requests that require certain permissions.

#### Request

```http
POST /authentication/login
{
  "username": "example",
  "password": "password"
}
```

#### Response

```http
{
  "token": "Bearer access token here"
}
```

### Current

The current user can be retrieved using the access token that was obtained after logging in.

#### Request

```http
GET /authentication/current
Authorization: Bearer <access token here>
```

#### Response

```http
{
  "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
  "username": "admin",
  "email": "test@codedead.com",
  "firstName": "Test",
  "lastName": "Test",
  "roles": [
    {
      "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
      "name": "DEFAULT",
      "description": "The default role",
      "permissions": [
        {
          "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
          "name": "CAN_UPDATE_SELF",
          "description": "The ability to update your own user"
        }
      ]
    }
  ]
}
```

## Users

Users can be managed using the following CRUD endpoints:

* `/users/`
* `/users/{id}`

### Create

Users can be created by other users with the appropriate authorizations.

#### Request

```http
POST /users/
{
  "username": "username",
  "email": "example@codedead.com",
  "firstName": "Jane",
  "lastName": "Doe",
  "password": "password",
  "roles": [
    "role id here"
  ]
}
```

#### Response

```http
{
  "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
  "username": "username",
  "email": "example@codedead.com",
  "firstName": "Jane",
  "lastName": "Doe",
  "enabled": true,
  "roles": [
    {
      "id": "role id here",
      "name": "DEFAULT",
      "description": "The default role",
      "permissions": [
        {
          "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
          "name": "CAN_UPDATE_SELF",
          "description": "The ability to update your own user"
        }
      ]
    }
  ]
}
```

### Read

#### Find a single user

##### Request

```http
GET /users/{id}
Authorization: Bearer <access token here>
```

##### Response

```http
{
  "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
  "username": "username",
  "email": "example@codedead.com",
  "firstName": "Jane",
  "lastName": "Doe",
  "enabled": true,
  "roles": [
    {
      "id": "role id here",
      "name": "DEFAULT",
      "description": "The default role",
      "permissions": [
        {
          "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
          "name": "CAN_UPDATE_SELF",
          "description": "The ability to update your own user"
        }
      ]
    }
  ]
}
```

#### Find all users

##### Request

```http
GET /users/
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
    "username": "username",
    "email": "example@codedead.com",
    "firstName": "Jane",
    "lastName": "Doe",
    "enabled": true,
    "roles": [
      {
        "id": "role id here",
        "name": "DEFAULT",
        "description": "The default role",
        "permissions": [
          {
            "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
            "name": "CAN_UPDATE_SELF",
            "description": "The ability to update your own user"
          }
        ]
      }
    ]
  }
]
```

#### Searching

##### Request

```http
GET /users/?text=example
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
    "username": "username",
    "email": "example@codedead.com",
    "firstName": "Jane",
    "lastName": "Doe",
    "enabled": true,
    "roles": [
      {
        "id": "role id here",
        "name": "DEFAULT",
        "description": "The default role",
        "permissions": [
          {
            "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
            "name": "CAN_UPDATE_SELF",
            "description": "The ability to update your own user"
          }
        ]
      }
    ]
  }
]
```

### Update

#### Request

```http
PUT /users/{id}
{
  "username": "username",
  "email": "example@codedead.com",
  "firstName": "John",
  "lastName": "Doe",
  "roles": [
    "role id here"
  ],
  "enabled": true
}
```

#### Response

```http
{
  "id": "d594989b-48bd-43d8-ab3e-d28671f145e6",
  "username": "username",
  "email": "example@codedead.com",
  "firstName": "John",
  "lastName": "Doe",
  "enabled": true,
  "roles": [
    {
      "id": "role id here",
      "name": "DEFAULT",
      "description": "The default role",
      "permissions": [
        {
          "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
          "name": "CAN_UPDATE_SELF",
          "description": "The ability to update your own user"
        }
      ]
    }
  ]
}
```

### Delete

#### Request

```http
DELETE /users/{id}
```

#### Response

```http
200 OK
```

## Roles

Roles can be managed using the following CRUD endpoints:

* `/roles/`
* `/roles/{id}`

### Create

#### Request

```http
{
  "name": "Role name",
  "description": "Role description",
  "permissions": [
    "permission id here"
  ]
}
```

#### Response

```http
{
  "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
  "name": "Role name",
  "description": "Role description",
  "permissions": [
    {
      "id": "permission id here",
      "name": "CAN_UPDATE_SELF",
      "description": "The ability to update your own user",
      "createdAt": "2023-08-01T00:16:26.911565688+00:00",
      "updatedAt": "2023-08-01T00:16:26.911565688+00:00"
    }
  ],
  "createdAt": "2023-08-01T00:16:27.223266792+00:00",
  "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
}
```

### Read

#### Find a single role

##### Request

```http
GET /roles/{id}
Authorization: Bearer <access token here>
```

##### Response

```http
{
  "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
  "name": "Role name",
  "description": "Role description",
  "permissions": [
    {
      "id": "permission id here",
      "name": "CAN_UPDATE_SELF",
      "description": "The ability to update your own user",
      "createdAt": "2023-08-01T00:16:26.911565688+00:00",
      "updatedAt": "2023-08-01T00:16:26.911565688+00:00"
    }
  ],
  "createdAt": "2023-08-01T00:16:27.223266792+00:00",
  "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
}
```

#### Find all roles

##### Request

```http
GET /roles/
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
    "name": "Role name",
    "description": "Role description",
    "permissions": [
      {
        "id": "permission id here",
        "name": "CAN_UPDATE_SELF",
        "description": "The ability to update your own user",
        "createdAt": "2023-08-01T00:16:26.911565688+00:00",
        "updatedAt": "2023-08-01T00:16:26.911565688+00:00"
      }
    ],
    "createdAt": "2023-08-01T00:16:27.223266792+00:00",
    "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
  }
]
```

#### Searching

##### Request

```http
GET /roles/?text=DEFAULT
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
    "name": "Default",
    "description": "Role description",
    "permissions": [
      {
        "id": "permission id here",
        "name": "CAN_UPDATE_SELF",
        "description": "The ability to update your own user",
        "createdAt": "2023-08-01T00:16:26.911565688+00:00",
        "updatedAt": "2023-08-01T00:16:26.911565688+00:00"
      }
    ],
    "createdAt": "2023-08-01T00:16:27.223266792+00:00",
    "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
  }
]
```

### Update

#### Request

```http
PUT /roles/{id}
Authorization: Bearer <access token here>
{
  "name": "Role name",
  "description": "Role description",
  "permissions": [
    "permission id here"
  ]
}
```

#### Response

```http
{
  "id": "16a639cc-2240-4d2f-8def-bea0a729dd9e",
  "name": "Role name",
  "description": "Role description",
  "permissions": [
    {
      "id": "permission id here",
      "name": "CAN_UPDATE_SELF",
      "description": "The ability to update your own user",
      "createdAt": "2023-08-01T00:16:26.911565688+00:00",
      "updatedAt": "2023-08-01T00:16:26.911565688+00:00"
    }
  ],
  "createdAt": "2023-08-01T00:16:27.223266792+00:00",
  "updatedAt": "2023-08-01T00:16:27.223266792+00:00"
}
```

### Delete

#### Request

```http
DELETE /roles/{id}
AUthorization: Bearer <access token here>
```

#### Response

```http
200 OK
```

## Permissions

Permissions can be managed using the following CRUD endpoints:

* `/permissions/`
* `/permissions/{id}`

### Create

#### Request

```http
POST /permissions/
Authorization: Bearer <access token here>
{
  "name": "CAN_UPDATE_SELF",
  "description": "The ability to update your own user"
}
```

#### Response

```http
{
  "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
  "name": "CAN_UPDATE_SELF",
  "description": "The ability to update your own user"
}
```

### Read

#### Find a single permission

##### Request

```http
GET /permissions/{id}
Authorization: Bearer <access token here>
```

##### Response

```http
{
  "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
  "name": "CAN_UPDATE_SELF",
  "description": "The ability to update your own user"
}
```

#### Find all permissions

##### Request

```http
GET /permissions/
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
    "name": "CAN_UPDATE_SELF",
    "description": "The ability to update your own user"
  },
  ...
]
```

#### Searching

##### Request

```http
GET /permissions/?text=CAN_UPDATE_SELF
Authorization: Bearer <access token here>
```

##### Response

```http
[
  {
    "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
    "name": "CAN_UPDATE_SELF",
    "description": "The ability to update your own user"
  },
  ...
]
```

### Update

#### Request

```http
PUT /permissions/{id}
Authorization: Bearer <access token here>
{
  "name": "CAN_UPDATE_SELF",
  "description": "The ability to update your own user"
}
```

#### Response

```http
{
  "id": "078bb9bf-21c4-4a5f-8f30-f7367a1de1b9",
  "name": "CAN_UPDATE_SELF",
  "description": "The ability to update your own user"
}
```

### Delete

#### Request

```http
DELETE /permissions/{id}
Authorization: Bearer <access token here>
```

#### Response

```http
200 OK
```
