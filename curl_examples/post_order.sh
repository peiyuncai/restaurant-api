#!/bin/bash

curl --location 'http://localhost:3030/orders' \
--header 'Content-Type: application/json' \
--data '{
    "table_id": 2,
    "menu_items": [
        {
            "menu_item_id": "433e36e8-f049-475a-8fa9-0b5453770f1a",
            "name": "Burger",
            "price": "855"
        },
        {
            "menu_item_id": "433e36e8-f049-475a-8fa9-0b5453770f1b",
            "name": "Fries",
            "price": "349"
        },
        {
            "menu_item_id": "433e36e8-f049-475a-8fa9-0b5453770f1c",
            "name": "Drink",
            "price": "350"
        },
        {
            "menu_item_id": "433e36e8-f049-475a-8fa9-0b5453770f1d",
            "name": "Ice Cream",
            "price": "477"
        }
    ]
}'
