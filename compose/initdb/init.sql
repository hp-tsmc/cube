-- initdb/init.sql

-- Create a new table
CREATE TABLE exampledb.users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(50) NOT NULL UNIQUE
);

-- Insert some example data
INSERT INTO exampledb.users (name, email) VALUES ('John Doe', 'john.doe@example.com');
INSERT INTO exampledb.users (name, email) VALUES ('Jane Smith', 'jane.smith@example.com');
INSERT INTO exampledb.users (name, email) VALUES ('Alice Johnson', 'alice.johnson@example.com');
