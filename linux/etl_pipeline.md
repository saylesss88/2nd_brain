---
id: etl_pipeline
aliases:
  - ETL Pipeline with Bash
tags: []
---

# ETL Pipeline with Bash

#Bash

Set up a dedicated directory for our ETL project:

```Bash
mkdir ~/etl_project
cd ~/etl_project
```

## Extracting Data

Data can come from various sources such as APIs, CSV files, or databases.

- In this example we'll extract data from a public API and CSV file.

Let's use curl to fetch data from a public API

```Bash
# Fetching data from a public API
curl -o data.json "https://api.example.com/data"
```

We can also use curl to download a CSV file from a remote server.

```Bash
# Downloading a CSV file
curl -o data.csv "https://example.com/data.csv"
```

- This will save the CSV file as data.csv in our working directory.

### Transforming Data

Data transformation is necessary to convert raw data into a format suitable for analysis or storage.

- This may involve parsing JSON, filtering CSV files, or cleaning text data.

`jq` is a powerful tool for working with JSON data. Let's use it to extract specific fields from our JSON file.

````Bash
# Parsing and extracting specific fields from JSON
```Bash
jq '.data[] | {id, name, value}' data.json > transformed_data.json
````

This command extracts the id, name, and value fields from each entry in
the JSON data and saves the result in transformed_data.json.

`awk` is a versatile tool for processing CSV files. We'll use it to extract specific columns from our CSV file.

-[NOTE]: The curl data.json fails for some reason but the .csv works

```Bash
# Extracting specific columns from CSV
awk -F, '{print $1, $3}' data.csv > transformed_data.csv
```

This command extracts the first and third columns from data.csv and saves them in transformed_data.csv.

`sed` is a stream editor for filtering and transforming text. We can use it to perform text replacements and clean up our data.

```Bash
# Replacing text in a file
sed 's/old_text/new_text/g' transformed_data.csv
```

This command replaces occurrences of old_text with new_text in transformed_data.csv.

#### Loading Data

Common destinations for loading data include databases and files.

Let's create a new SQLite database and a table to hold our data.

```Bash
# Creating a new SQLite database and table
sqlite3 etl_database.db "CREATE TABLE data (id INTEGER PRIMARY KEY, name TEXT, value REAL);"
```

This command creates a db file named etl_database.db and a table named data with three columns.

Next, we'll insert our transformed data into the SQLite db.

```Bash
# Inserting data into SQLite database
sqlite3 etl_database.db <<EOF
.mode csv
.import transformed_data.csv data
EOF
```

This block of commands sets the mode to CSV and imports transformed_data.csv into the data table.

We can verify that the data has been inserted correctly by querying the db.

```Bash
# Querying the database
sqlite3 etl_database.db 'SELECT * FROM data;'
```
