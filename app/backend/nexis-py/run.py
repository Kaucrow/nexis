import uvicorn
from nexis_py import create_app
from nexis_py.settings import get_settings
from nexis_py.colors import red

app = create_app()
settings = get_settings()

if __name__ == "__main__":
    if settings.application.protocol == "https":
        try:
            uvicorn.run(
                "run:app",
                host=settings.application.host,
                port=settings.application.port,
                ssl_certfile="cert/cert.pem",
                ssl_keyfile="cert/key.pem",
            )
        except FileNotFoundError:
            print(
                red("[ ERR ] ") + "The app protocol was set to `https` but the CA cert could not be found.\n"
                "Ensure there's a `cert.pem` and `key.pem` file inside the `cert` directory."
            )
            exit(1)
    else:
        uvicorn.run(
            "run:app",
            host=settings.application.host,
            port=settings.application.port
        )