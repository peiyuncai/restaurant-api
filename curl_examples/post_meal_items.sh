#!/bin/bash

curl --location 'http://localhost:3030/meal-items' \
--header 'Content-Type: application/json' \
--data '{
    "table_id": 2,
    "menu_items": [
        {
            "menu_item_id": "433e36e8-f049-475a-8fa9-0b5453770f1e",
            "name": "tea",
            "price": "123"
        },
        {
            "menu_item_id": "433e36e8-f049-475a-8fa9-0b5453770f1f",
            "name": "cheese cake",
            "price": "749"
        }
    ]
}'