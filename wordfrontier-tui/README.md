# wordfrontier-tui

Text User Interface for `wordfrontier`.

## Design Notes

-   What is a "word frontier"?  A word frontier, relative to a set of "known words" is a set of
    sentences in the target language which contain a specified number of unknown words.  Thus it
    is necessarily parametric.  Examples:
    -   A word frontier showing all sentences with exactly 1 unknown word.
    -   A word frontier showing 200 sentences with 1 or 2 unknown words.
    -   A word frontier showing a collection of sentences whose unknown words come from a fixed
        set of vocab words (say selected based on a theme, or from a book's chapter's vocab list)
-   Conceptually, there are two collections of data:
    -   Corpus (sentences, translations, and associated data).  This does contain any user-specific data.
    -   Known words.  This only contains user-specific data; it catalogs the words the user knows, and later
        will catalog other related user-specific data.
-   UI elements
    -   Generic text content list
        -   Filter parameters
            -   Max number of results to show
            -   Result page
            -   Random shuffle toggle (not sure how this is possible via SQL)
            -   Random seed
            -   Filter by content matching (ideally, using a loose search, tolerant of accented chars, etc)
        -   UI list element
    -   Known word list
        -   Base is a generic text content list UI element showing the known word list
        -   UI element to add/remove/update words to known word list
        -   Currently selected word -> show sentences containing that word in another UI panel
    -   Corpus word list
        -   Base is a generic text content list UI element showing the corpus word list
        -   UI element to show UI for corpus sentences containing a particular word
        -   Currently selected word -> show sentences containing that word in another UI panel
        -   Currently selected word ->
            -   UI to add word to known word list
            -   UI to remove word from known word list
            -   If word is known, highlight that word in known word list
    -   Corpus sentence list
        -   Base is a generic text content list UI element showing the corpus sentence list
        -   Additional UI filter parameter for allowable range of unknown word count, so that a word frontier
        -   Currently selected sentence -> show translation(s) in another UI panel
    -   Corpus translation list
        -   Base is a generic text content list UI element showing the translations for a particular sentence

## To-dos

-   Implement frequency-ordering of word frontier list.
-   Instead of having a separate panel for "Sentence Words", highlight the words in the "Word Frontier"
    panel itself, using different colors based on their being in the known_words table or not, and give the
    ability to add/remove words from that panel.  Use a background color to indicate the selected word,
    and use foreground colors to indicate level of knowledge.  N-grams could also be selected by expanding
    the background color left and right to encompass the N-gram, though this is a more ill-defined thing.
-   Make it so that re-computing the word frontier requires an explicit action.  This is so that there is
    continuity in the content as the user is reviewing words and adding/removing them from known_words.
-   Add a command to add all words in a sentence to the known_words table.
-   Generate n-grams (2, 3, and maybe 4), analyze the frequency, and select some significant top portion
    of them, as these could/should represent common sentence fragments.  Make n-grams a "knowable" primitive,
    so those are an object of learning too.

## License and Attributions

This work is Copyright (C) 2021 Victor Dods and is released under the [MIT License](LICENSE).

Note that the `wordfrontier-tui` codebase started life as a modification of the
[`crossterm_demo`](https://github.com/fdehau/tui-rs/blob/master/examples/crossterm_demo.rs) example in the
[`tui`](https://crates.io/crates/tui) crate which is MIT-Licensed and is Copyright (c) 2016 Florian Dehau.
