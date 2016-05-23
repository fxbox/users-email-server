# users-email-server

Simple server that allow clients to send user management related emails.

Only one endpoint is available so far:

```ssh
POST /v1/invitation HTTP/1.1
Content-Type: application/json
{
  "email": "user@domain.org",
  "url": "https://local.<hash>.knilxof.org/whatever"
}
```

This endpoint allows the client to send a invitation to a user to allow
her to activate her account. The body of the request must contain the email
of the invitation recipient and a URL from a domain that must be known to the
server (i.e. https://<hash>.knilxof.org/<path>) and that will be added to the
email body.

In the future, we may want to add new endpoints for extra functionality
such as password reset emails, for instance.

## Usage

```ssh
cargo run -- --email-server smtp.gmail.com --email-user username@gmail.com --email-password apassword --email-from Manolo -u "https.*\b(local.*.knilxof.org.*)|https.*\b(remote.*.knilxof.org.*)"
```

Note: If you want to use Gmail, you'll need to [allow less secure apps to access your account](https://support.google.com/accounts/answer/6010255?hl=en)
