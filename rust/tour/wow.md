Memory details:

    The first rule of references prevents data races. What's a data race? A data race when reading from data has the possibility of being out of sync due to the existence of a writer to the data at the same time. This happens often in multi-threaded programming.
    The second rule of references prevents the misuse of references that refer to non-existent data (called dangling pointers in C).
