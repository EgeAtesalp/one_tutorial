# one_tutorial
My implementation of (https://git.sr.ht/~gruberb/onetutorial) for training purposes on Rust

Topics include:
1)Send a GET request to https://httpbin.org/ip and print the result to the console.
2)Send out a request to CMC and fetch the price of BTC.
3)Read the API key from an .env file.
4)Get the name, symbol, price and 7day of the BTC and store it in a struct.
5)Pass the list of currencies to fetch via the CLI.
6)Save the result in a CSV file.
7)In case the API returns an error, write it out to a log file and abort the application.
8)In addition to the coin prices, fetch the price of a random ETF and store it also in a struct.
9)Add a rows in your Google Sheet with ISN, amount of coins, price, total and a row with the total value of your portfolio.
10)Instead of saving the result to a CSV, update your Google sheet.
11)Move out your business logic in different modules.
12)Build your Rust code and move the binary to a server and run it from there.
13)Send out an E-Mail with the coin and ETF overview and redeploy your application/binary.