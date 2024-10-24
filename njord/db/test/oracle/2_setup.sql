-- Step 1: Ensure you are in the correct container (Pluggable Database)
ALTER SESSION
SET
    CONTAINER = FREEPDB1;

-- Step 2: Create the user in that specific container
CREATE USER njord_user IDENTIFIED BY njord_password QUOTA UNLIMITED ON USERS;

-- Step 3: Grant privileges to the new user
GRANT CONNECT,
RESOURCE,
CREATE TABLE TO njord_user;

-- Step 4: Commit changes to make sure the user is visible
COMMIT;

-- Table: users
CREATE TABLE njord_user.users (
    id NUMBER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    -- Auto incrementing primary key for the user ID
    username VARCHAR2(255) NOT NULL,
    -- Username field
    email VARCHAR2(255) NOT NULL,
    -- Email field
    address VARCHAR2(255) -- Address field
);

-- Table: categories
CREATE TABLE njord_user.categories (
    id NUMBER PRIMARY KEY,
    -- Primary key for categories
    name VARCHAR2(255) NOT NULL -- Name of the category
);

-- Table: products
CREATE TABLE njord_user.products (
    id NUMBER PRIMARY KEY,
    -- Primary key for products
    name VARCHAR2(255) NOT NULL,
    -- Product name
    description CLOB,
    -- Product description (using CLOB for large text)
    price NUMBER(10, 2) NOT NULL,
    -- Price with up to two decimal places
    stock_quantity NUMBER NOT NULL,
    -- Stock quantity
    category_id NUMBER NOT NULL,
    -- Foreign key to categories (one-to-one relationship)
    discount NUMBER(5, 2) DEFAULT 0.00,
    -- Discount field with default value
    CONSTRAINT fk_category FOREIGN KEY (category_id) REFERENCES njord_user.categories(id) -- Foreign key constraint to categories table
);