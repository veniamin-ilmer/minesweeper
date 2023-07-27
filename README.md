# minesweeper

Experimental Minesweeper game built with Iced

Run with:

    cargo run

Features:

* Randomly places mines, and writes numbers accordingly.
* Clicking a blank space recusively opens up the sides and corners.
* Detects if you won or lost the game, lets you restart the game.

Missing Features:

* Right clicking detection is not available in Iced, so you can't flag a bomb.
* Double clicking detection is not available in Iced, so you can't reveal neighboring cells this way.
* Clock or mine counter
