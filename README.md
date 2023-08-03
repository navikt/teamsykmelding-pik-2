# teamsykmelding-pik-2
This project contains the application code and infrastructure for teamsykmelding-pik-2 based on this kotlin project https://github.com/navikt/teamsykmelding-pik

## Technologies used
* Rust
* Cargo
* Docker


### Prerequisites
Make sure you have the rust installed using this command:
#### Rust
```bash script
rustc --version
```

#### Cargo
Make sure you have cargo installed using this command:
```bash script
cargo --version
```

### Build
Build the code without running it
```bash script
cargo build
```

### Test
Build the code and run all the tests
```bash script
cargo test
```

#### Running the application locally

#####  Create docker image of app
Creating a docker image should be as simple as
``` bash
docker build -t rustapp .
```

#### Running a docker image
``` bash
docker run --rm -it -p 8080:8080 rustapp
```

##### 🧪 Testing the applications endpoints

Request to is_alive
```bash script
curl --location --request GET 'http://0.0.0.0:8080/internal/is_alive'
```

Request to is_alive
```bash script
curl --location --request GET 'http://0.0.0.0:8080/internal/is_ready'
```

### Contact

This project is maintained by [navikt/teamsykmelding](CODEOWNERS)

Questions and/or feature requests? Please create an [issue](https://github.com/navikt/teamsykmelding-pik-2/issues)

If you work in [@navikt](https://github.com/navikt) you can reach us at the Slack
channel [#team-sykmelding](https://nav-it.slack.com/archives/CMA3XV997)
