# Squirtle

Squirtle is a dead simple CLI for playing the wordle. The basic idea is to use simple heuristics to narrow the search space of viable words as rapidly as possible.

## Usage
Squirtle assumes it is starting from a blank slate. It suggests a word to input. When you input this word, you tell squirtle what the result of the guess was. For example, if you guess `PALMS` and got the following result

![An example of playing PALMS. P is yellow, A is gray, L is yellow, M is green, S is yellow.](example.png)

you should input mnmym (m means "move", n means "no", y means "yes"). This is a little awkward, but green and gray both start with g, so what are you gonna do.

This repo includes a word list derived from Webster's dictionary.

## Strategy

The high level idea is to narrow the search space as much as possible with each guess.

Given information about past guesses, squirtle filters down the set of all words to the set of all viable words. It then measures the frequency of letters in the viable word set. It then guesses a viable word whose letters appear in the highest number of viable words. This is a simple heuristic that seems to work pretty well in practice.

## Performance

On a blank board, the viable word list includes 5379 words. In one sample run, after 1 guess (squirtle suggests "AROSE" as a good first guess), this was reduced to 158 viable words. After 2 guesses, this was reduced to 9 viable words. It got the right word on the third guess.