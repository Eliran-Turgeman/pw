# pw

`pw` is a command-line tool designed for secure password management. Utilizing modern cryptography, `pw` ensures the secure storage of passwords and offers functionalities for generating strong passwords, retrieving stored passwords, and checking if your passwords have been compromised in known data breaches.

## Features

- **Set Password**: Securely store a new password under a specified key. If the key exists, its password will be overwritten.
- **Get Password**: Retrieve and display the password stored under a specified key.
- **Analyze Password**: Check if the password(s) stored under the specified key (or all passwords if no key is provided) have been compromised in known data breaches using the "Have I Been Pwned" API.
- **Generate Password**: Generate a strong, random password of a specified length. The generated password is displayed but not stored automatically; use the 'set' command to store it if desired.

## Installation

Ensure Rust and Cargo are installed on your machine. Follow the installation steps here: [Install Rust](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```bash
git clone https://your-repository-url.git
cd pw
cargo build --release
```

The executable will be located in `./target/release/.`

## Usage

### Set a New Password

```
pw set --key <KEY> --value <PASSWORD>
```

### Get a Password

```
pw get --key <KEY>
```

### Analyze a Password

To analyze a specific password:

```
pw analyze --key <KEY>
```

To analyze all stored passwords:

```
pw analyze
```

### Generate a Strong Password

To generate a password of default length (12 characters):

```
pw generate
```

To specify the length:

```
pw generate --length <LENGTH>
```

To directly store the generated password:

```
pw generate --key <KEY> --length <LENGTH>
```