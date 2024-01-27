# Puzzle solve
## 1. Introduction
The goal of this project is to present the development and implementation of an algorithm for reconstructing an image through puzzles. The approach we will explore involves analyzing the characteristics of each puzzle piece, considering the topology of the image, and applying algorithmic techniques to efficiently assemble the original image. Expected results include precise and fast image reconstruction, with a significant emphasis on preserving details and structure. This algorithm can serve as a foundation for further research and application in a broader spectrum of areas where image reconstruction through puzzles is of paramount importance.
<p align="center">
  <img src="https://github.com/NikolaKalinic/jigsaw-puzzle/blob/main/img.png" alt="Alt Text">
</p>

## 2. Algorithm Description:
The approach to solving the puzzle image assembly problem is based on an algorithm that efficiently combines image fragments to form the final picture. This algorithm, developed and implemented, goes through several key steps to achieve accurate assembly of the image from smaller components.
- **Loading the Final Image:** The first step of the algorithm involves loading the target image, which serves as the reference point for reconstruction. This image acts as the foundation for creating the end result.
- **Loading Other Parts (Puzzle Pieces):** After loading the final image, the algorithm needs to load other images into a vector, providing an excellent opportunity for parallelization, which will be discussed later.
- **Initializing an Empty Image:** Based on the dimensions of the final image, the algorithm creates an empty image that will eventually represent the assembled result. This empty image initially has no information, and the goal is to gradually fill it with smaller pieces.
- **Finding the Best Puzzle for Each Position:** The algorithm then traverses the original image, analyzing each position to find the best matching piece. This process relies on calculating the Euclidean distance.
- **Placing Puzzles on the Empty Image:** Once the appropriate puzzle is found, the algorithm places it in the corresponding position on the empty image. After placement, the puzzle is removed from the vector of all puzzles to avoid reuse.
- **Iterative Process:** This process repeats until all puzzles are placed on the empty image, adhering to the selective removal of puzzles from the vector to ensure each puzzle is used only once.
## 3. Getting Started
To get started with the puzzle solve project, follow these steps:
1. Clone the repository:

    ```bash
    $ git clone https://github.com/NikolaKalinic/jigsaw-puzzle.git
    ```

2. Navigate to the project directory:

    ```bash
    $ cd jigsaw-puzzle
    ```

3. Run the following command to build and run your project:

    ```bash
    $ cargo run "1"
    ```
