from fastapi.responses import JSONResponse

def SuccessResponse(msg: str, status=200) -> JSONResponse:
    return JSONResponse(
        content={
            "message": msg
        },
        status_code=status
    )

def ErrorResponse(msg: str, status=500) -> JSONResponse:
    return JSONResponse(
        content={
            "error": msg
        },
        status_code=status
    )