CREATE TABLE Users (
    uuid SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL
    password VARCHAR(255) NOT NULL,
);

CREATE TABLE Clients (
    userUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (userUuid) REFERENCES Users(uuid),
    phone VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    gender VARCHAR(255) NOT NULL
);

CREATE TABLE Employees (
    userUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (userUuid) REFERENCES Users(uuid),
    phone VARCHAR(255) NOT NULL,
    age INT NOT NULL
);






