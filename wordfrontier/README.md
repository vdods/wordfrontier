# wordfrontier

An experimental language training tool.

## Training Corpuses

Download selected language's sentences:
-   https://tatoeba.org/en/downloads

## To-dos

-   Use tatoeba.org sentence IDs
-   Incorporate tatoeba.org translations
-   Use https://ichi.moe/cl/qr/?q=%E6%97%A5%E6%9B%9C%E6%97%A5%E3%81%AB%E5%AF%BF%E5%8F%B8%E3%82%92%E9%A3%9F%E3%81%B9%E3%81%BE%E3%81%99&r=htr as a nice sentence parsing website.
-   For now, the corpus DB and the known words DB have to be in the same DB, since the queries
    have to involve both of them.
    -   Generalize `known_words` table into `word_sets` table, where different collections
        of words can be created, e.g.
        -   Known words
        -   Known words of different categories
        -   In-progress words (seen before but not mastered)
        -   The various word sets of different users potentially
-   Should be able to promote/demote a word in a sentence into a different word set, e.g.
    into known words, or in-progress words, etc.
-   Should be able to find all (or N random) sentences containing a given word, where the
    idea is that a precise definition isn't necessary, it can be derived intuitively from
    its actual usage in the sentences (and of course the corresponding translations).
-   Whenever UI is a thing, it would be cool to be able to highlight the words in a sentence
    based on which word set(s) they're a member of.  E.g. render known words in grey,
    in-progress words in teal, and unknown words in yellow.  Clicking on a word would
    bring up a side panel of sentences that contain that word (and possibly their translations),
    as well as controls for promoting/demoting that word.
-   Use SQLite's `ATTACH` feature to separate out DBs:
    -   Corpus content (langs, sentences, words, sentence_memberships, translations)
    -   Word sets
-   Refactor import function to use prepared statements to run faster.
-   Allow for multiple corpus imports, where duplicate sentences are tolerated (but they should have
    the expected sentence IDs).  E.g. importing `deu <=> eng` and `spa <=> eng`, the English sentences
    that overlap should be tolerated and checked.

## License and Attributions

This work is Copyright (C) 2021 Victor Dods and is released under the [MIT License](LICENSE).
