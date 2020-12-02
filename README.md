# ONVotes

### See how your representatives vote in Ontario's Parliament


## What is this?

All proceedings in Ontario's Parliament are recorded and published in a document called Hansard, including votes. ONVotes serves to index and organize the votes published by the parliament, to make this vital democratic information more readily available to constituents.

## Parts of ONVotes

This repository is broadly split into 4 parts

1. `analyzer` — `analyzer` is the code responsible for extracting votes from Hansard. Once a day, a copy of this code will run to analyze the previous day's proceedings, extract the relevant information, and publish it to the data served by the website.
2. `common` — `common` is simply a library containing struct definitions for JSON that is used throughout ONVotes to serialize data
3. `server` — `server` is the backend of ONVotes, it keeps a record of the votes gathered by `analyzer`, and hosts the website to make the content accessible. It is written with [actix-web](https://github.com/actix/actix-web)
4. `src` — `src` is the source directory of the frontend of ONVotes, it is written in Rust with [seed](https://github.com/seed-rs/seed)
