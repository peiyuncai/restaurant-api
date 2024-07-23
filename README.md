# Simple Restaurant API

### There are 6 APIs.

Please check [swagger file](./swagger.yaml) for the complete contract

| API                                       | Description                      |
|-------------------------------------------|----------------------------------|
| POST /orders                              | create new order                 |
| GET /orders/{table-id}                    | get order by table id            |
| DELETE /orders/{table-id}                 | delete order by table id         |
| POST /meal-items                          | add meal items to existing order |
| GET /meal-items/{table-id}/{meal-item-id} | get meal item                    |
| DELETE /meal-items                        | delete meal items                |

Use Postman would be easier for testing the APIs. Can find collections [here](./RAPI.postman_collection.json). 
<br> Or there are curl examples. Can find them [here](./curl_examples)

### How to start application
```
cargo run
cargo run -- 3
make run
make run ARGS="3"

#first parameter is size of chef thread pool; default is 2
```
### How to run unit tests
```
cargo test
make test
```

### General Testing Scenarios

Can run in any order.
However, to run happy case, There are some assumptions need to know.
#### Assumption
1. We can only create order if there is no order in received or preparing status for the same table
2. We can only add meal items if there exists order for the table
3. We can only remove meal item if it's not being prepared or completed
4. We can only remove order if none of the meal item is being prepared or completed
5. We always do soft delete, meaning data is not really removed

#### Steps
1. start application
2. run **Post /orders** to create order
    - menu_item_id: could be any uuid number
    - name: could be any
    - price: if price is 50.95, then use String 5095 here
3. run **POST /meal-items** to add more meal items to the created order
    - table_id: should be same as previous one; otherwise, get not found error
    - menu_item_id: could be any uuid number
    - name: could be any
    - price: if price is 50.95, then use String 5095 here
4. run **GET /orders/{table-id}**
    - table_id: should be same as previous one; otherwise, get not found error
5. run **GET /meal-items/{table-id}/{meal-item-id}**
    - table_id: should be same as previous one
    - meal_item_id: you can find all the meal-item-id from response of step 1, 2, or 3. Can pick any of them.
    - if providing invalid table_id or meal-item-id, will get not found error
6. run **DELETE /orders/{table-id}**
    - table_id:
        - given valid table_id and if order is being prepared or completed, will fail to delete order
        - using invalid table_id, will get not found error
7. run **DELETE /meal-items**
    - if there are any valid meal_item_ids, those meal items which are not yet being prepared will be deleted and others
      will be just omitted
    - if table_id and none of meal_item_ids are valid, will get error message

### Application Modules

1. main is the entry point of application
2. handlers have all the handlers handling 6 APIs respectively
3. libraries have thread_pool, job, and worker. These are used to create a chef thread pool, and we have a channel to
   queue the cooking job(we use meal item's cooking time as thread's sleeping time). The number of thread should be same
   as number of chef we want.
4. models have all the models to CRUD order, meal item, and menu item (though menu item is not really used)
5. repositories have all the repositories for order and menu. Though menu is not really used. All the data change can
   only be done via repositories. No data change can be done via data model. I use DashMap as data store here.

### Application Logic

We start server on 127.0.0.1:3030 by running **cargo run** or *cargo run -- {pool_size}* and can serve each request
asynchronously. I did not limit the threads here.
When we receive POST /order or POST /meal-items, the thread will put meal item cooking jobs in channel.
On the other hand, we have a chef thread pool to consume cooking job and prepare the meal item.
Here give an example. Assume we have 2 thread in the thread pool which means we have 2 chefs.
When client issue a request with 3 meal_items for table 1, these 3 items will be sent to channel(FIFO).
If client issue a request again with 2 meal_items for table 2, these 2 items will be sent to the same channel as well.
The chef thread will loop consuming the meal_item from channel and based on meal_item's cooking_item, chef thread will go
sleep to simulate busy cooking the meal item until time is elapsed. Then it will wake up and consume another meal item from channel.
Before it goes to sleep, it will update meal_item's status as **preparing**, so client can't cancel it.
After thread wakes up, it will then mark meal item status as **completed**, and client can't cancel it either.
Other meal items queued in the channel whose status is the default **received**, so client can still cancel it.
If chef thread gets the meal from channel and connect db checking meal's status and find it's canceled, then it will return
without further processing.
There is cooking_time_upper_bound_in_min in order model which is the sum of non-removed and non-completed meal items'
cooking time.
Since there could be more than one chef thread to process meal, the actual cooking time could be less, but we use
upper_bound here to denote max required time.

### Application Improvement Areas

Due to time constraints, I have not yet finished everything but this version should cover basic functions.
There are still many improvements space. I will note down here.

1. Currently only critical components have unit tests added. 
2. Should separate out DB model from domain model, currently we use the same model for both for simplicity(though make testing more difficult)
3. Error handling can be more concise and unified
4. Separate request and response models from handler modules
5. Menu models are actually dummy models. Otherwise, we should validate received Menu with DB's Menu or have different
   implementation to avoid data manipulation, ex. price manipulation
6. Limit the number of threads serving API requests
7. Archive completed order to another table (another data structure). Now when order is completed or canceled, order can be overridden.
8. Once order gets started preparing, we can't cancel order as a whole. We can improve to have more granular control where maybe we can cancel those meals not yet being prepared.
9. API Path and method design did not follow best practice. Can be improved.
10. Currently, we can delete data multiple times, this can be improved.

