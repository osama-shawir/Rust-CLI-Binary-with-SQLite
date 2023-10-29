# SQLite ETL and CRUD in Rust

![SQLite ETL and CRUD](pipeline.png)

The SQLite ETL and CRUD project is a Rust-based command-line application designed for performing ETL (Extract, Transform, Load) operations and CRUD (Create, Read, Update, Delete) actions on an SQLite database. It offers a versatile way to interact with your database, including querying, inserting, updating, and deleting records.

## Features

- **Query Database:** Retrieve and display data from the SQLite database, with the ability to apply filtering and sorting to the results.

- **Insert Data:** Add new records to the database, allowing you to populate it with relevant information.

- **Update Records:** Modify existing records, ensuring your database remains up-to-date with the latest data.

- **Delete Records:** Remove unwanted records from the database, maintaining data accuracy.

## How to Use

To utilize this project for your SQLite database needs, follow the steps below:

1. **Clone the Repository** or run this on Codespaces
2. Running the Program:

Ensure that you have Rust and Cargo installed. You can download and install them from the official website.

After installation, you can run the program using the cargo run command, along with the appropriate command-line arguments to specify the action you want to perform. Here are some example commands:

### Query the Database:

To retrieve and display data, run:

```bash
cargo run -- query
```

This will execute a query and display the requested information. In our case we set it to display the information of the ten people with the highest salaries in the database.

### Insert Data:

To add new records to the database, run:

bash
Copy code
cargo run -- insert your-ssn your-annual-income
Replace your-ssn and your-annual-income with the appropriate values.

### Update Records:

To modify existing records, run:

``` bash
cargo run -- update your-ssn new-annual-income
```

Replace your-ssn with the SSN of the record you want to update and new-annual-income with the updated income value.

### Delete Records:

To remove records from the database, run:

```bash
cargo run -- delete your-ssn
```

Replace your-ssn with the SSN of the record you want to delete.

Enjoy the convenience of performing ETL and CRUD operations on your SQLite database with this Rust project. Keep your data organized and up-to-date effortlessly.
