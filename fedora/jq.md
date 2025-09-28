# jq

## What is jq?
`jq` is a lightweight and powerful command-line tool that allows you to manipulate and transform JSON data effortlessly.
- It provides many functions for querying, filtering, formatting, and modifying JSON documents.

### Basic Usage

Let's start with a simple JSON doc:
```JSON
{
  "name": "John Doe",
  "age": 30,
  "city": "New York"
}
```
Suppose you want to extract the "name" field. You can use jq like this:
```bash
echo '{"name": "John Doe", "age": 30, "city": "New York"}' | jq '.name'
```
This command will output:
`"John Doe"`

#### Filtering JSON Arrays

`jq` is also great for working with JSON arrays. Suppose you have an array of products:
```JSON
[
  {
    "name": "Laptop",
    "price": 999
  },
  {
    "name": "Smartphone",
    "price": 599
  },
  {
    "name": "Tablet",
    "price": 349
  }
]
```
To filter this array to only include products with a price less than $500,you can use the following command:
```bash
echo '[{"name": "Laptop", "price": 999}, {"name": "Smartphone", "price": 599}, {"name": "Tablet", "price": 349}]' | jq '.[] | select(.price < 500)'
```
The output will be:
```bash
{
  "name": "Tablet",
  "price": 349
}
```
