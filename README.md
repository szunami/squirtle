# Squirtle

Squirtle is a dead simple CLI for playing the wordle. The basic idea is to use simple heuristics to narrow the search space of viable words as rapidly as possible.

## Usage
Squirtle assumes it is starting from a blank slate. It suggests a word to input. When you input this word, you tell squirtle what the result of the guess was. For example, if you guess `PALMS` and got the following result

![An example of playing PALMS. P is yellow, A is gray, L is yellow, M is green, S is yellow.](example.png)

you should input mnmym (m means "move", n means "no", y means "yes"). This is a little awkward, but green and gray both start with g, so what are you gonna do.

This repo includes a word list derived from Webster's dictionary.

## Performance

On a blank board, this word list includes 5379 words. In one run, after 1 guess, this was reduced to 158 viable words. After 2 guesses, this was reduced to 9 viable words. It got the right word on the third guess.