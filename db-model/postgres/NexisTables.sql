CREATE TABLE Users (
    uuid SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE Clients (
    clientUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (clientUuid) REFERENCES Users(uuid),
    phone VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    gender VARCHAR(255) NOT NULL
);

CREATE TABLE Employees (
    employeeUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (employeeUuid) REFERENCES Users(uuid),
    phone VARCHAR(255) NOT NULL,
    age INT NOT NULL
);

CREATE TABLE Admin (
    adminUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (adminUuid) REFERENCES Users(uuid)
);

CREATE TABLE Schedule (
    uuid SERIAL PRIMARY KEY,
    employeeUuid SERIAL,
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    enterDate DATE NOT NULL,
    exitDate DATE NOT NULL
);

CREATE TABLE Jobs (
    uuid SERIAL PRIMARY KEY,
    jobName VARCHAR(255) NOT NULL
);

CREATE TABLE EmployeeJob (
    scheduleUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (scheduleUuid) REFERENCES Schedule(uuid),
    jobUuid SERIAL,
    FOREIGN KEY (jobUuid) REFERENCES Jobs(uuid)
);

CREATE TABLE Stores (
    uuid SERIAL PRIMARY KEY,
    storeName VARCHAR(255) NOT NULL,
    storeNum INT NOT NULL,
    storeFloor INT NOT NULL,
    storeLenght INT NOT NULL,
    storeWidth INT NOT NULL,
    storeHeight INT NOT NULL
);

CREATE TABLE StoreJobs (
    storeUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (storeUuid) REFERENCES Stores(uuid),
    jobUuid SERIAL,
    FOREIGN KEY (jobUuid) REFERENCES Jobs(uuid)
);