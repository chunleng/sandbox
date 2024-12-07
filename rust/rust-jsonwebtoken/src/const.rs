// We use the following EdDSA key for decoding:
//   {
//     "keys": [
//       {
//         "use": "sig",
//         "kty": "OKP",
//         "kid": "d6e82f14-3743-4602-8a16-0eb1a44d6937",
//         "crv": "Ed25519",
//         "alg": "EdDSA",
//         "x": "BBOqS6Gh17K1fu8m8TEteGyymZLif2rl7FceK3o8KfY",
//         "d": "BXuu4qvL7Pv9FjkYHI4INYAr1eD3pzuLL4y4UsYgeLc"
//       }
//     ]
//   }
// Used other Ory Kratos to make this JWT below as rust does not yet have a library that can work
// with EdDSA encoding with ease at the time of writing.
pub const EDDSA_JWT: &str = "eyJhbGciOiJFZERTQSIsImtpZCI6ImQ2ZTgyZjE0LTM3NDMtNDYwMi04YTE2LTBlYjFhNDRkNjkzNyIsInR5cCI6IkpXVCJ9.eyJkYXRhIjp7ImVtYWlsIjoiYUBhLmNvbSJ9LCJleHAiOjE3MzM1NjkyMjQsImlhdCI6MTczMzU2OTE2NCwiaXNzIjoiaHR0cDovLzEyNy4wLjAuMTo0NDMzLyIsImp0aSI6IjQ3NGU2MThhLTQ3MDUtNDc2ZC1hZTRlLWM4Y2Y4YWNmMDgxNiIsIm5iZiI6MTczMzU2OTE2NCwic2lkIjoiNWI3N2YyYWMtNDdiNy00NjMzLWJiYjYtYmJhYmVlMjcyNzgyIiwic3ViIjoiNzE3ZDdiY2MtNzlmMi00NTUyLTgwZDAtMmQzMjU3OTUzMjVmIn0.SqIW-NZmpWPlayxOGe7I7wyK6UBEMrdS86RhJMMkyx7s1lsvLHQS69wGafjvF64LPGSTK4HPNbKpp4dhA9lpAw";
pub const EDDSA_JWK_X: &str = "BBOqS6Gh17K1fu8m8TEteGyymZLif2rl7FceK3o8KfY";
