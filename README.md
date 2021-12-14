# twenty_forty_eight_in_rust  
- ### VIDEO GAME: 2048
- ### Group CMSC388Z BEST GAMERS EVER 🎮 👾 🏀 🏈 ⚽

### Team Members (by alphabetical order):
- #### Abdoul K Tou-Kone
- #### Dhanvee Ivaturi
- #### Franklin Nkokam Ngongang

![Screen Shot Of action_1](https://github.com/ntuifranklin/twenty_forty_eight_in_rust/blob/9ec069dc82038c47c5c9f176c226034751facd0e/rust_2048/screenshots/action_1.png =250x250)
![Screen Shot Of action_2](https://github.com/ntuifranklin/twenty_forty_eight_in_rust/blob/9ec069dc82038c47c5c9f176c226034751facd0e/rust_2048/screenshots/action_1.png =250x250 )
![Screen Shot Of action_3](https://github.com/ntuifranklin/twenty_forty_eight_in_rust/blob/9ec069dc82038c47c5c9f176c226034751facd0e/rust_2048/screenshots/action_1.png =250x250)

## Appetizer
2048 is a single-player game based on sliding tiles and was written by Italian web developer Gabriele Cirulli and published on GitHub.
It was initially written in Javascript and CSS.
The main goal of this game is to slide numbered tiles of multiples of 2, on a grid to combine same numbered ones,
then to create a tile with the number 2048.
Our 100% goal is to create a 2048 game in a console with colorful tiles.
Our 75% goal is to create a working black and white 2048 game that we can play in the command line.
Our 125% goal  is to create a 2048 game with a nice looking GUI where the tiles change colors based on the numbers and have a smooth animation
when moving on the board.
The trivial challenge we can think about is how to create a working graphical user interface to meet the 125% goal.
Another issue we could face is how to build the game in the terminal. The team is still researching how it will be done.

### Precoding Tasks
- Research libraries for building graphical user interface (GUI) in rust. Here is a list of rust libraries officially available to build GUI in rust : https://lib.rs/gui
- ##### Research command line libraries in rust and the feasibility of building the game in the terminal.
- ##### Write down the algorithm in plain english how the 2048 game works.
- ##### Transform the algorithm in plain english into user stories and component stories, with the fewest details as possible.
- Elaborate the interaction between the user stories and the components.
- ##### Based on the written user stories in plain english, sub divide the program into components(classes and methods).
- ##### Create a github repository.  Add developers to the github repository.

### Coding Tasks
- ##### For each class, all the developers will work on writing the signature for each method.
This will prevent the issues stated on item #2 of the [#notes](https://github.com/cmsc388z/lectures/blob/main/lecture-8.md#project-introduction) section in the project proposal document , from happening.
- ##### Synchronously write the unit and integration tests for the classes, and functions.  
- ##### Build the matrix page containing the squares. This will be a class/object.
- ##### Create a library that’ll shift the matrix up/down/left/right
- ##### Build the GUI
- ##### Make the GUI display the matrix of squares, colored accordingly
- ##### Make the GUI listen to keypresses and modify the matrix by repositioning the squares according to the instructions of the algorithm.
- ##### Implement each class and write the signatures of each class methods without the code
- ##### Write unit tests for each method written for the classes before starting to write any implementation code.


Cited references:
Ferri-Benedetti, Fabrizio (26 March 2014). "The creator of 2048 tells us the secret behind the game's success". Softonic. Retrieved 13 November 2020.
