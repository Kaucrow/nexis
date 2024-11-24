# Download Swagger UI
Invoke-WebRequest -Uri "https://github.com/swagger-api/swagger-ui/archive/master.zip" -OutFile "swagger-ui-master.zip"

# Extract the contents
Expand-Archive -Path "swagger-ui-master.zip" -DestinationPath "."

# Create swagger-ui directory and copy necessary files
New-Item -ItemType Directory -Force -Path "swagger-ui"
Copy-Item -Path "swagger-ui-master\dist\*" -Destination "swagger-ui" -Recurse

# Clean up temporary files
Remove-Item -Path "swagger-ui-master.zip"
Remove-Item -Path "swagger-ui-master" -Recurse

Write-Host "Swagger UI setup complete. Files are in the 'swagger-ui' directory."