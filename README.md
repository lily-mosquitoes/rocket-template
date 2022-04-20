# Rocket Template

This is a template for a rocket application!

## How to run

### Locally

You'll need [rust installed](https://www.rust-lang.org/tools/install), as well as a [PostgreSQL](https://www.postgresql.org/) database setup of your choice.

Create a `.env` file with the necessary variables set (detailed in `.env.template`).

Run with `cargo run`. :sparkles:

### With Docker

You'll need the [docker engine](https://docs.docker.com/engine/install/) and [compose](https://github.com/docker/compose).

Create a `.env` file with the necessary variables set (detailed in `.env.template`).

Simply run `docker compose up`. :sparkles:

If you'd still like to run the Rocket application locally but dockerize the postgres instance, you may run the provided file for the database like: `docker compose -f docker-postgres.yml up`. Then run `cargo run`!

## Copyright

This code is licensed under the GNU Affero General Public License version 3 or later. See [LICENSE](LICENSE) or https://www.gnu.org/licenses/agpl-3.0.en.html.

Made with :heart: by Lílian
