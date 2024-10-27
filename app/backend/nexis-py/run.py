from nexis_py import create_app
from nexis_py.settings import get_settings
from nexis_py.colors import red

app = create_app()
settings = get_settings()

if __name__ == "__main__":
    if settings.application.protocol == "https":
        try:
            app.run(
                ssl_context=(
                    "cert/cert.pem","cert/key.pem"
                ),
                host=settings.application.host,
                port=settings.application.port
            )
        except FileNotFoundError:
            print(
                red("[ ERR ] ") + "The app protocol was set to `https` but the CA cert could not be found.\n"
                "Ensure there's a `cert.pem` and `key.pem` file inside the `cert` directory."
            )
            exit(1)
    else:
        app.run(
            host=settings.application.host,
            port=settings.application.port
        )