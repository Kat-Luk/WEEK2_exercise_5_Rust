Task 5

In this exercise, you will again create new files in a module called receipt. Make a folder called receipt with files content.rs, product.rs and mod.rs. Make the two files public in mod.rs. In product.rs, make a structure called StoreProduct, and a function called create_products() that creates a vector of type StoreProduct and stores 3 variables of type StoreProduct with the values ​​given in the image and returns an array with all 3 in the given order. StoreProduct has a name and a total price.



The content.rs file contains most of the code in this task.
Use StoreProduct from the product.rs file.



Make constant variables for the three products as shown.
Make a struct called ReceiptContent with two attributes: "products" which is a vector of type StoreProduct and "store" which is of type String.

Make a loop that asks the user to either 1) add to cart 2) remove the newest item or 3) buy the current items in the cart. If the user decides to add to cart, ask the user which product they want to add and print all the options on their own lines using the print function. By choosing 1, 2 or 3, the user input matches the products in the image above.

When the user adds a product using the function, adds the product to a variable of type ReceiptContent.

When the user wants to remove the latest product, remove the latest product from the ReceiptContent variable.

When the user wants to buy all the products they have selected, calculate the total price of the products using the function you created called complete_purchase() with the return type of empty Result. The receipt should be outputted to a textfile called "receipt.txt".

Example:


Below is a receipt printout in addition to an example run. Write the information on the receipt, how many items were bought, how much they cost and the final price, the name of the store.
Example run:

| 1) Add to cart | 2) Remove most recent product | 3) Purchase |
1
Which product would you like to add?
1) Zbox 720 | Price - 600
2) GPU - AND Random RT6600 | Price - 200
3) Potato | Price - 1
1
| 1) Add to cart | 2) Remove most recent product | 3) Purchase |
1
Which product would you like to add?
1) Zbox 720 | Price - 600
2) GPU - AND Random RT6600 | Price - 200
3) Potato | Price - 1
2
| 1) Add to cart | 2) Remove most recent product | 3) Purchase |
1
Which product would you like to add?
1) Zbox 720 | Price - 600
2) GPU - AND Random RT6600 | Price - 200
3) Potato | Price - 1
3
| 1) Add to cart | 2) Remove most recent product | 3) Purchase |
1
Which product would you like to add?
1) Zbox 720 | Price - 600
2) GPU - AND Random RT6600 | Price - 200
3) Potato | Price - 1
1
| 1) Add to cart | 2) Remove most recent product | 3) Purchase |
2
| 1) Add to cart | 2) Remove most recent product | 3) Purchase |
3
Thank you for your purchase!

Receipt printing example (print only those products of which at least 1 copy has been purchased):

Imaginary Town General Store
------------------------------
Zbox 720 (1) - 600€
GPU - AND Random RT6600 (1) - 200€
Potato (1) - 1€
------------------------------
Final price: 801€
------------------------------
