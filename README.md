# Rocket Template

This is a template for a rocket application!

## How to run

- Set up a [PostgreSQL](https://www.postgresql.org/) database.
    - If you have [docker-ce](https://docs.docker.com/engine/install/) you can provision a database with the [compose](https://github.com/docker/compose) tool:
        - Create a `.compose` file with the necessary variables set (see [`.compose.template`](.compose.template)).
        - Run `docker compose -f docker-postgres.yml up`.


- Create a `.env` file with the necessary variables set (see [`.env.template`](.env.template)).

- Install `libpq-dev` and [Rust](https://www.rust-lang.org/tools/install), if you haven't already, then run `cargo run`. :sparkles:
    - Alternatively, if you have [docker-ce](https://docs.docker.com/engine/install/) you can build and deploy an image with the provided [`Dockerfile`](Dockerfile) using the [compose](https://github.com/docker/compose) tool:
        - Run `docker compose -f docker-rocket.yml up`.

## Copyright

This code is licensed under the GNU Affero General Public License version 3 or later. See [LICENSE](LICENSE) or https://www.gnu.org/licenses/agpl-3.0.en.html.

Made with :heart: by Lílian
