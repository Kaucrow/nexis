from fastapi.responses import JSONResponse
from . import router

@router.get("/health-check")
def health_check():
    return JSONResponse(
        content="Python API is safe and healthy! :)",
        status_code=200
    )