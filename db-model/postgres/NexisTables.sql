CREATE TABLE Users (
    uuid UUID PRIMARY KEY,
    nameUser VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE Clients (
    clientUuid UUID PRIMARY KEY,
    FOREIGN KEY (clientUuid) REFERENCES Users(uuid),
    phone VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    gender VARCHAR(255) NOT NULL
);

CREATE TABLE Employees (
    employeeUuid UUID PRIMARY KEY,
    FOREIGN KEY (employeeUuid) REFERENCES Users(uuid),
    phone VARCHAR(255) NOT NULL,
    age INT NOT NULL
);

CREATE TABLE Admins (
    adminUuid UUID PRIMARY KEY,
    FOREIGN KEY (adminUuid) REFERENCES Users(uuid)
);

CREATE TABLE Schedule (
    uuid UUID PRIMARY KEY,
    employeeUuid UUID,
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    enterDate DATE NOT NULL,
    exitDate DATE NOT NULL,
    checkedIn DATE NOT NULL,
    checkedOut DATE NOT NULL
);

CREATE TABLE Jobs (
    uuid UUID PRIMARY KEY,
    jobName VARCHAR(255) NOT NULL
);

CREATE TABLE Stores (
    uuid UUID PRIMARY KEY,
    storeName VARCHAR(255) NOT NULL,
    storeNum INT NOT NULL,
    storeFloor INT NOT NULL,
    storeLenght INT NOT NULL,
    storeWidth INT NOT NULL,
    storeHeight INT NOT NULL
);

CREATE TABLE StoreJobs (
    uuid UUID PRIMARY KEY,
    storeUuid UUID,
    FOREIGN KEY (storeUuid) REFERENCES Stores(uuid),
    jobUuid UUID,
    FOREIGN KEY (jobUuid) REFERENCES Jobs(uuid),
    payPerHour FLOAT(2) NOT NULL,
    payPerWeek FLOAT(2) NOT NULL
);

CREATE TABLE EmployeeJob (
    scheduleUuid UUID PRIMARY KEY,
    FOREIGN KEY (scheduleUuid) REFERENCES Schedule(uuid),
    storeJobUuid UUID,
    FOREIGN KEY (storeJobUuid) REFERENCES StoreJobs(uuid)
);

CREATE TABLE StoreOwners (
    adminUuid UUID,
    FOREIGN KEY (adminUuid) REFERENCES Admins(adminUuid),
    storeUuid UUID,
    FOREIGN KEY (storeUuid) REFERENCES Stores(uuid),
    incomePercentage INT NOT NULL
);

CREATE TABLE FoodType (
    uuid UUID PRIMARY KEY,
    typeName VARCHAR(255) NOT NULL
    
);

CREATE TABLE Food (
    uuid UUID PRIMARY KEY,
    typeFoodUuid UUID,
    FOREIGN KEY (typeFoodUuid) REFERENCES FoodType(uuid),
    foodName VARCHAR(255) NOT NULL,
    foodPrice FLOAT(2) NOT NULL,
    pricePerKg FLOAT(2) NOT NULL
);

CREATE TABLE foodreviews (
    reviewUuid UUID,
    FOREIGN KEY (reviewUuid) REFERENCES Food(uuid),
    stars FLOAT(2) NOT NULL
);  

CREATE TABLE FoodLot (
    uuid UUID PRIMARY KEY,
    enterDate DATE NOT NULL,
    expiry DATE NOT NULL
);

CREATE TABLE FoodLotItems (
    foodLotUuid UUID,
    FOREIGN KEY (foodLotUuid) REFERENCES FoodLot(uuid),
    foodUuid UUID,
    FOREIGN KEY (foodUuid) REFERENCES Food(uuid),
    quantityInStock INT NOT NULL
);

CREATE TABLE LibraryItems (
    uuid UUID PRIMARY KEY,
    itemName VARCHAR(255) NOT NULL,
    itemPrice FLOAT(2) NOT NULL
);

CREATE TABLE libraryreviews (
    reviewUuid UUID,
    FOREIGN KEY (reviewUuid) REFERENCES LibraryItems(uuid),
    stars FLOAT(2) NOT NULL
);

CREATE TABLE BookPublishers (
    uuid UUID PRIMARY KEY,
    publisherName VARCHAR(255) NOT NULL
);


CREATE TABLE Books (
    libraryItemUuid UUID PRIMARY KEY,
    FOREIGN KEY (libraryItemUuid) REFERENCES LibraryItems(uuid),
    publisherUuid UUID,
    FOREIGN KEY (publisherUuid) REFERENCES BookPublishers(uuid),
    numPages INT NOT NULL,
    bookEdition VARCHAR(255) NOT NULL
);

CREATE TABLE Authors (
    uuid UUID PRIMARY KEY,
    authorName VARCHAR(255) NOT NULL
);

CREATE TABLE BookAuthors (
    bookUuid UUID,
    FOREIGN KEY (bookUuid) REFERENCES Books(libraryItemUuid),
    authorUuid UUID,
    FOREIGN KEY (authorUuid) REFERENCES Authors(uuid)
);

CREATE TABLE Audiences (
    uuid UUID PRIMARY KEY,
    audienceName VARCHAR(255) NOT NULL
);

CREATE TABLE BookAudiences (
    bookUuid UUID,
    FOREIGN KEY (bookUuid) REFERENCES Books(libraryItemUuid),
    audienceUuid UUID,
    FOREIGN KEY (audienceUuid) REFERENCES Audiences(uuid)
);

CREATE TABLE Genres (
    uuid UUID PRIMARY KEY,
    genreName VARCHAR(255) NOT NULL
);

CREATE TABLE BookGenres (
    bookUuid UUID,
    FOREIGN KEY (bookUuid) REFERENCES Books(libraryItemUuid),
    genreUuid UUID,
    FOREIGN KEY (genreUuid) REFERENCES Genres(uuid)
);

CREATE TABLE ClothesTypes (
    uuid UUID PRIMARY KEY,
    typeName VARCHAR(255) NOT NULL
);

CREATE TABLE Clothes (
    uuid UUID PRIMARY KEY,
    typeClothesUuid UUID,
    FOREIGN KEY (typeClothesUuid) REFERENCES ClothesTypes(uuid),
    clothesName VARCHAR(255) NOT NULL,
    clothesPrice FLOAT(2) NOT NULL,
    clothesBrand VARCHAR(255) NOT NULL,
    clothesSize VARCHAR(10) NOT NULL,
    clothesGender VARCHAR(255) NOT NULL,
    clothesAge VARCHAR(255) NOT NULL
);

CREATE TABLE clothesreviews (
    reviewUuid UUID,
    FOREIGN KEY (reviewUuid) REFERENCES Clothes(uuid),
    stars FLOAT(2) NOT NULL
);

CREATE TABLE Colors (
    uuid UUID PRIMARY KEY,
    colorName VARCHAR(255) NOT NULL
);

CREATE TABLE ClothesColors (
    clothesUuid UUID,
    FOREIGN KEY (clothesUuid) REFERENCES Clothes(uuid),
    colorUuid UUID,
    FOREIGN KEY (colorUuid) REFERENCES Colors(uuid)
);

CREATE TABLE TechCpu (
    uuid UUID PRIMARY KEY,
    cpuBrand VARCHAR(255) NOT NULL,
    cpuModel VARCHAR(255) NOT NULL,
    cpuPrice FLOAT(2) NOT NULL
    
);

CREATE TABLE TechGpu (
    uuid UUID PRIMARY KEY,
    gpuBrand VARCHAR(255) NOT NULL,
    gpuModel VARCHAR(255) NOT NULL,
    gpuPrice FLOAT(2) NOT NULL,
    dedicated BOOLEAN NOT NULL
);

CREATE TABLE Tech (
    uuid UUID PRIMARY KEY,
    uuidCpu UUID,
    FOREIGN KEY (uuidCpu) REFERENCES TechCpu(uuid),
    uuidGpu UUID,
    FOREIGN KEY (uuidGpu) REFERENCES TechGpu(uuid),
    techName VARCHAR(255) NOT NULL,
    techPrice FLOAT(2) NOT NULL,
    techBrand VARCHAR(255) NOT NULL,
    techType VARCHAR(255) NOT NULL,
    techModel VARCHAR(255) NOT NULL
);

CREATE TABLE techreviews (
    reviewUuid UUID,
    FOREIGN KEY (reviewUuid) REFERENCES Tech(uuid),
    stars FLOAT(2) NOT NULL
);

CREATE TABLE TechColors (
    techUuid UUID,
    FOREIGN KEY (techUuid) REFERENCES Tech(uuid),
    colorUuid UUID,
    FOREIGN KEY (colorUuid) REFERENCES Colors(uuid)
);

CREATE TABLE ClientName (
    uuid UUID PRIMARY KEY,
    clientName VARCHAR(255) NOT NULL
);

CREATE TABLE SalesInPerson (
    uuid UUID,
    dateSalesInPerson DATE,
    PRIMARY KEY (uuid, dateSalesInPerson),
    clientUuid UUID,
    FOREIGN KEY (clientUuid) REFERENCES Clients(clientUuid)

);

CREATE TABLE SalesOnline (
    uuid UUID,
    dateSalesOnline DATE,
    PRIMARY KEY (uuid,dateSalesOnline),
    clientUuid UUID,
    FOREIGN KEY (clientUuid) REFERENCES Clients(clientUuid)
);

CREATE TABLE Sales (
    uuid UUID PRIMARY KEY,
    salesInPersonUuid UUID,
    salesOnlineUuid UUID,
    dateSalesInPerson DATE,
    dateSalesOnline DATE,

    FOREIGN KEY (salesInPersonUuid, dateSalesInPerson) REFERENCES SalesInPerson(uuid, dateSalesInPerson),
    FOREIGN KEY (salesOnlineUuid, dateSalesOnline) REFERENCES SalesOnline(uuid, dateSalesOnline),

    CHECK (
        (
            salesInPersonUuid IS NOT NULL AND dateSalesInPerson IS NOT NULL AND
            salesOnlineUuid IS NULL AND dateSalesOnline IS NULL
        ) OR
        (
            salesInPersonUuid IS NULL AND dateSalesInPerson IS NULL AND
            salesOnlineUuid IS NOT NULL AND dateSalesOnline IS NOT NULL
        )
    )
);

CREATE TABLE SalesTech (
    saleUuid UUID,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    techUuid UUID,
    FOREIGN KEY (techUuid) REFERENCES Tech(uuid),
    employeeUuid UUID,   
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    price FLOAT(2) NOT NULL,
    returned BOOLEAN NOT NULL
);

CREATE TABLE SalesClothes (
    saleUuid UUID,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    clothesUuid UUID,
    FOREIGN KEY (clothesUuid) REFERENCES Clothes(uuid),
    employeeUuid UUID,   
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    price FLOAT(2) NOT NULL,
    returned BOOLEAN NOT NULL
);

CREATE TABLE SalesFood (
    saleUuid UUID,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    foodUuid UUID,
    FOREIGN KEY (foodUuid) REFERENCES Food(uuid),
    employeeUuid UUID,
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    price FLOAT(2) NOT NULL,
    returned BOOLEAN NOT NULL
);

CREATE TABLE SalesLibrary (
    saleUuid UUID,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    libraryUuid UUID,
    FOREIGN KEY (libraryUuid) REFERENCES LibraryItems(uuid),
    employeeUuid UUID,
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    price FLOAT(2) NOT NULL,
    returned BOOLEAN NOT NULL
);

CREATE TABLE PaymentMethods (
    uuid UUID PRIMARY KEY,
    paymentMethod VARCHAR(255) NOT NULL
);

CREATE TABLE Payments (
    saleUuid UUID PRIMARY KEY,
    FOREIGN KEY (saleUuid) REFERENCES Sales(uuid),
    uuidMethod UUID,
    FOREIGN KEY (uuidMethod) REFERENCES PaymentMethods(uuid),
    amount INT NOT NULL
);

CREATE TABLE EmployeeStats (
    employeeUuid UUID,
    FOREIGN KEY (employeeUuid) REFERENCES Employees(employeeUuid),
    dateES DATE NOT NULL,
    PRIMARY KEY (employeeUuid, dateES),
    salesPerWeek INT NOT NULL
);

CREATE TABLE TechStats (
    techUuid UUID,
    dateTechStats DATE,
    PRIMARY KEY (techUuid, dateTechStats),
    salesPerWeek FLOAT(2) NOT NULL
);

CREATE TABLE TechBrandStats (
    uuid UUID PRIMARY KEY,
    nameTBS VARCHAR(255) NOT NULL
);

CREATE TABLE TechModelsStats (
    uuid UUID,
    dateTechModelsStats DATE NOT NULL,
    PRIMARY KEY (uuid, dateTechModelsStats),
    brandUuid UUID,
    FOREIGN KEY (brandUuid) REFERENCES TechBrandStats(uuid),
    nameTMS VARCHAR(255) NOT NULL,
    percentageTMS FLOAT(2) NOT NULL
);

CREATE TABLE FoodStats (
    foodUuid UUID PRIMARY KEY,
    FOREIGN KEY (foodUuid) REFERENCES Food(uuid)

);

CREATE TABLE LibraryStats (
    libraryUuid UUID,
    dateLS DATE NOT NULL,
    FOREIGN KEY (libraryUuid) REFERENCES LibraryItems(uuid),
    PRIMARY KEY (libraryUuid, dateLS),
    salesPerWeek FLOAT(2) NOT NULL,
    bookOfTheWeek VARCHAR(255) NOT NULL
);

CREATE TABLE LibraryGenderStats (
    dateLGS DATE PRIMARY KEY,
    malePercentage FLOAT(2) NOT NULL,
    femalePercentage FLOAT(2) NOT NULL
);

CREATE TABLE ClothesStats(
    clothesUuid UUID,
    dateCS DATE NOT NULL,
    FOREIGN KEY (clothesUuid) REFERENCES Clothes(uuid),
    PRIMARY KEY (clothesUuid, dateCS),
    salesPerWeek FLOAT(2) NOT NULL
);

CREATE TABLE SizeClothesStats (
    dateSCS DATE PRIMARY KEY,
    percentageSCS FLOAT(2) NOT NULL,
    sizeSCS VARCHAR(255) NOT NULL
);

CREATE TABLE GenderClothesStats (
    dateGCS DATE PRIMARY KEY,
    malePercetage FLOAT(2) NOT NULL,
    femalePercetage FLOAT(2) NOT NULL
);

CREATE TABLE ExpiredFoodStats (
    foodLotUuid UUID PRIMARY KEY,
    FOREIGN KEY (foodLotUuid) REFERENCES FoodLot(uuid),
    quantity INT NOT NULL
);

CREATE TABLE GenreLibraryStats (
    uuidGenre UUID,
    dateGLS DATE NOT NULL,
    FOREIGN KEY (uuidGenre) REFERENCES Genres(uuid),
    PRIMARY KEY (uuidGenre, dateGLS),
    percentageGLS FLOAT(2) NOT NULL
);

