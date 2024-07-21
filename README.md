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
`cargo run` or 
`cargo run -- 4` (can change 4 to any number which is the size of chef thread pool, default is 2)

### General Testing Scenarios: (Please find full contract in swagger)

Can run in any order.
However, to run happy case, step1. need to go before step2 at least.

0. run `cargo run` or `cargo run -- $pool_size`
1. run **Post /orders** to create order
    - menu_item_id: could be any uuid number
    - name: could be any
    - price: if price is 50.95, then use String 5095 here
2. run **POST /meal-items** to add more meal items to the created order
    - table_id: should be same as previous one; otherwise, throw not found error
    - menu_item_id: could be any uuid number
    - name: could be any
    - price: if price is 50.95, then use String 5095 here
3. run **GET /orders/{table-id}**
    - table_id: should be same as previous one; otherwise, throw not found error
4. run **GET /meal-items/{table-id}/{meal-item-id}**
    - table_id: should be same as previous one
    - meal_item_id: you can find all the meal-item-id from response of step 1, 2, or 3. Can pick any of them.
    - if providing invalid table_id or meal-item-id, will get not found error
5. run **DELETE /orders/{table-id}**
    - table_id:
        - given valid table_id and if order is being prepared or completed, will fail to delete order
        - using invalid table_id, will have not found error
6. run **DELETE /meal-items**
    - if there are any valid meal_item_ids, those meal items which are not yet being prepared will be deleted and others
      will be just omitted
    - if table_id and none of meal_item_ids are valid, will get error message

### Application Modules

1. main is the entry point of application
2. handlers have all the handlers handling each API request
3. libraries have thread_pool, job, and worker. These are used to create a chef thread pool, and we have a channel to
   queue the cooking job(we use meal item's cooking time as thread's sleeping time). The number of thread should be same
   as number of chef we have.
4. models have all the models to CRUD order and item
5. repositories have all the repositories for order and menu. Though menu is not really used. All the data change can
   only be done via repositories. No data change can be done via data model. I use DashMap as data store here.

### Application Logic

We start server on 127.0.0.1:3030 by running **cargo run** or *cargo run -- {pool_size}* and can serve each request
asynchronously. I did not limit the threads here.
On the other hand, we create a chef thread pool to consume and prepare the meal item.
Let's give an example. Assume we have 2 thread in the thread pool which means we have 2 chefs.
When client issue a request with 3 meal_items for table 1, these 3 items will be sent to channel(FIFO).
If client issue a request again with 2 meal_items for table 2, these 2 items will be sent to the same channel as well.
The chef thread will loop get the meal_item from channel and based on meal_item's cooking_item, chef thread will go
sleep until time is elapsed.
Before it goes to sleep, it will update meal_item's status as **preparing**, so client can't cancel it.
After thread wakes up, it will then mark meal item status as **completed**, so client can't cancel it either.
Other meal items queued in the channel whose status will be default **received**, so client can still cancel it.
If thread get the meal from channel and connect db to check meal's status and find it's canceled, then it will return
without further processing.
There is cooking_time_upper_bound_in_min in order model which is the sum of non-removed and non-completed meal items'
cooking time.
Since there could be more than chef thread to process meal, the actual cooking time could be less, but we use
upper_bound here to denote max required time.
Some Assumption: I assume we only add new order when order is finished for that table,
which means there is no check in place and order can be overridden with new one when passing same table_id.
However, there are checks on meal_items where it can't be canceled when it already started being prepared or already
completed.
Same for order, we can't remove it once check starts to prepare one of its meal items.

### Application Todo

Due to time constraints, I have not yet finished everything but this version should cover basic functions.
I will try to add unit tests once I finish documentations.
There are still many improvements space. I will note down here.

1. Unit tests are to be added
2. Error handling can be more concise and unified
3. Separate request and response models from handler files
4. Move common functions out from handlers
5. Menu models are actually dummy models. Otherwise, we can validate received Menu with DB's Menu or have different
   implementation to avoid request data manipulation
6. Limit the number of thread serving API requests
7. Archive completed order to another data store
8. Avoid order override when it's not completed yet
9. Improve order cancellation, maybe we can cancel those meals not yet being prepared.

