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

CREATE TABLE Admins (
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
    storeJobUuid SERIAL,
    FOREIGN KEY (storeJobUuid) REFERENCES StoreJobs(uuid)
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
    uuid SERIAL PRIMARY KEY,
    storeUuid SERIAL,
    FOREIGN KEY (storeUuid) REFERENCES Stores(uuid),
    jobUuid SERIAL,
    FOREIGN KEY (jobUuid) REFERENCES Jobs(uuid),
    payPerHour INT NOT NULL,
    payPerWeek INT NOT NULL
);

CREATE TABLE StoreOwners (
    adminUuid SERIAL,
    FOREIGN KEY (adminUuid) REFERENCES Admins(adminUuid),
    storeUuid SERIAL,
    FOREIGN KEY (storeUuid) REFERENCES Stores(uuid),
    incomePercentage INT NOT NULL
);

CREATE TABLE Food (
    uuid SERIAL PRIMARY KEY,
    typeFoodUuid SERIAL,
    FOREIGN KEY (typeFoodUuid) REFERENCES FoodType(uuid),
    foodName VARCHAR(255) NOT NULL,
    foodPrice INT NOT NULL
);

CREATE TABLE FoodType (
    uuid SERIAL PRIMARY KEY,
    typeName VARCHAR(255) NOT NULL,
    pricePerKg INT NOT NULL
);

CREATE TABLE FoodLot (
    uuid SERIAL PRIMARY KEY,
    enterDate DATE NOT NULL,
    expiry DATE NOT NULL
);

CREATE TABLE FoodLotItems (
    foodLotUuid SERIAL,
    FOREIGN KEY (foodLotUuid) REFERENCES FoodLot(uuid),
    foodUuid SERIAL,
    FOREIGN KEY (foodUuid) REFERENCES Food(uuid),
    quantityInStock INT NOT NULL
);

CREATE TABLE LibraryItems (
    uuid SERIAL PRIMARY KEY,
    itemName VARCHAR(255) NOT NULL,
    itemPrice INT NOT NULL
);

CREATE TABLE Books (
    libraryItemUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (libraryItemUuid) REFERENCES LibraryItems(uuid),
    publisherUuid SERIAL,
    FOREIGN KEY (publisherUuid) REFERENCES BookPublishers(uuid),
    numPages INT NOT NULL,
    bookEdition VARCHAR(255) NOT NULL
);

CREATE TABLE BookPublishers (
    uuid SERIAL PRIMARY KEY,
    publisherName VARCHAR(255) NOT NULL
);

CREATE TABLE Authors (
    uuid SERIAL PRIMARY KEY,
    authorName VARCHAR(255) NOT NULL
);

CREATE TABLE BookAuthors (
    bookUuid SERIAL,
    FOREIGN KEY (bookUuid) REFERENCES Books(libraryItemUuid),
    authorUuid SERIAL,
    FOREIGN KEY (authorUuid) REFERENCES Authors(uuid)
);

CREATE TABLE Audiences (
    uuid SERIAL PRIMARY KEY,
    audienceName VARCHAR(255) NOT NULL
);

CREATE TABLE BookAudiences (
    bookUuid SERIAL,
    FOREIGN KEY (bookUuid) REFERENCES Books(libraryItemUuid),
    audienceUuid SERIAL,
    FOREIGN KEY (audienceUuid) REFERENCES Audiences(uuid)
);

CREATE TABLE Genres (
    uuid SERIAL PRIMARY KEY,
    genreName VARCHAR(255) NOT NULL
);

CREATE TABLE BookGenres (
    bookUuid SERIAL,
    FOREIGN KEY (bookUuid) REFERENCES Books(libraryItemUuid),
    genreUuid SERIAL,
    FOREIGN KEY (genreUuid) REFERENCES Genres(uuid)
);

CREATE TABLE Clothes (
    uuid SERIAL PRIMARY KEY,
    clothesName VARCHAR(255) NOT NULL,
    clothesPrice INT NOT NULL,
    clothesBrand VARCHAR(255) NOT NULL,
    clothesSize VARCHAR(10) NOT NULL,
    clothesGender VARCHAR(255) NOT NULL,
    clothesAge VARCHAR(255) NOT NULL
);

CREATE TABLE Colors (
    uuid SERIAL PRIMARY KEY,
    colorName VARCHAR(255) NOT NULL
);

CREATE TABLE ClothesColors (
    clothesUuid SERIAL,
    FOREIGN KEY (clothesUuid) REFERENCES Clothes(uuid),
    colorUuid SERIAL,
    FOREIGN KEY (colorUuid) REFERENCES Colors(uuid)
);

CREATE TABLE Tech (
    uuid SERIAL PRIMARY KEY,
    uuidCpu SERIAL,
    FOREIGN KEY (uuidCpu) REFERENCES TechCpu(uuid),
    uuidGpu SERIAL,
    FOREIGN KEY (uuidGpu) REFERENCES TechGpu(uuid),
    techName VARCHAR(255) NOT NULL,
    techPrice INT NOT NULL,
    techBrand VARCHAR(255) NOT NULL,
    techType VARCHAR(255) NOT NULL,
    techModel VARCHAR(255) NOT NULL
);

CREATE TABLE TechColors (
    techUuid SERIAL,
    FOREIGN KEY (techUuid) REFERENCES Tech(uuid),
    colorUuid SERIAL,
    FOREIGN KEY (colorUuid) REFERENCES Colors(uuid)
);

CREATE TABLE TechCpu (
    uuid SERIAL PRIMARY KEY,
    cpuBrand VARCHAR(255) NOT NULL,
    cpuModel VARCHAR(255) NOT NULL,
    cpuPrice INT NOT NULL,
    soldSeparately BOOLEAN NOT NULL
);

CREATE TABLE TechGpu (
    uuid SERIAL PRIMARY KEY,
    gpuBrand VARCHAR(255) NOT NULL,
    gpuModel VARCHAR(255) NOT NULL,
    gpuPrice INT NOT NULL,
    dedicated BOOLEAN NOT NULL
);

CREATE TABLE Sales (
    uuid SERIAL PRIMARY KEY,
    clientUuid SERIAL,
    FOREIGN KEY (clientUuid) REFERENCES Clients(clientUuid),
    salesName VARCHAR(255) NOT NULL,
    date DATE NOT NULL
);

CREATE TABLE Payments (
    saleUuid SERIAL PRIMARY KEY,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    uuidMethod SERIAL,
    FOREIGN KEY (uuidMethod) REFERENCES PaymentMethods(uuid),
    amount INT NOT NULL
);

CREATE TABLE PaymentMethods (
    uuid SERIAL PRIMARY KEY,
    paymentMethod VARCHAR(255) NOT NULL
);

CREATE TABLE SalesTech (
    saleUuid SERIAL,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    techUuid SERIAL,
    FOREIGN KEY (techUuid) REFERENCES Tech(uuid),
    employeeUuid SERIAL,   
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    returned BOOLEAN NOT NULL
);

CREATE TABLE SalesClothes (
    saleUuid SERIAL,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    clothesUuid SERIAL,
    FOREIGN KEY (clothesUuid) REFERENCES Clothes(uuid),
    employeeUuid SERIAL,   
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    returned BOOLEAN NOT NULL
);