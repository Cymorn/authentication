Rust Authentication Backend

This project is a simple authentication backend built with Rust and Axum.
I built it to understand how authentication works behind the scenes and to practice writing real backend code in Rust.

The focus of this project is security, clean structure, and backend fundamentals.




What this project does

Hashes passwords securely using Argon2

Uses salt so the same password never produces the same hash

Verifies passwords during login

Generates JWT tokens after successful login

Uses stateless authentication (no sessions stored on the server)

Exposes simple HTTP endpoints using Axum



Why I built this

I wanted to go beyond tutorials and actually understand:

How password hashing really works

Why salting passwords is important

How JWT authentication works internally

How to structure a Rust backend project properly

This project helped me learn how real backend services handle authentication.



Tech Stack

Rust (2024 edition)

Axum – web framework

Argon2 – password hashing

JWT (jsonwebtoken crate) – authentication tokens

Serde – JSON serialization

UUID & Chrono – user IDs and token expiration


project structure
src/
 ├── main.rs
 └── utils/
     ├── mod.rs
     ├── password.rs   # password hashing & verification
     └── jwt.rs        # JWT creation & validation





Authentication approach

This project uses stateless authentication with JWTs.

The server signs a token after login

The token contains the user ID and expiration time

The server does not store sessions

On future requests, the token can be verified using the secret key

This approach is scalable and commonly used in real-world systems.




Future improvements

Add user registration

Store users in a database (PostgreSQL)

Protect routes using JWT middleware

Add role-based access control

Move secrets to environment variables





Notes

This is a learning project, but it follows real backend practices.
The goal is to keep improving it as I grow as a backend developer.
