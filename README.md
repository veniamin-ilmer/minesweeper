# minesweeper

Experimental Minesweeper game built with Iced

Run with:

    cargo run

Features:

* Randomly places mines, and writes numbers accordingly. Numbers are colored.
* Clicking a blank space recusively opens up the sides and corners.
* Detects if you won or lost the game, lets you restart the game.
* Right clicking a cell flagged a bomb. There is a bomb counter.

Missing Features:

* No double clicking detection, so you can't reveal neighboring cells this way.
* No clock
