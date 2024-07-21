#!/bin/bash

# need to change meal_item_ids
curl --location --request DELETE 'http://localhost:3030/meal-items' \
--header 'Content-Type: application/json' \
--data '{
    "table_id": 2,
    "meal_item_ids": [
        "ee5c8739-10a8-4b56-9c3c-7104dbfd286f"
    ]
}'