# Rust MUD Adventure

What is a MUD?

> “A MUD (/mʌd/; originally Multi-User Dungeon, with later variants Multi-User Dimension and Multi-User Domain)[1][2] is a multiplayer real-time virtual world, usually text-based. MUDs combine elements of role-playing games, hack and slash, player versus player, interactive fiction, and online chat. Players can read or view descriptions of rooms, objects, other players, non-player characters, and actions performed in the virtual world. Players typically interact with each other and the world by typing commands that resemble a natural language.”
>
> — https://en.wikipedia.org/wiki/MUD

## History

This is a project I've been working off and on again for a five or six years at this point, with the project essentially being started from scratch each time I pick it up again. Each iteration has usually been an attempt to learn a specific implementation strategy or framework.

## Goals

In this interation, the primary goal is to have a working text adventure like environment that is as data driven as possible. Historically, depending on the specific iterations implementation there have been some speedbumps in getting the client and server to work smoothly together during the development process. To work around that and aid in the development process, this iteration is being written with the goal of being run both as a server and a singleplayer experience.

## Current Status

Currently, the project can be run and guides the user through account and character creation, before placing the player in the world where they can move around and interact with it.

### ToDo

- Loading and saving player data to the database.
- Server functionality for the project.
- Rework how bonuses work for heritages. (Possibly make them feats instead.)
- Features in Rooms.
- Resolve inconsistent line breaks between different commands.

# Documentation

## Loading Data

Currently, the Game struct when it is initialized tries to load data from the "data/tad-rooms.tsv" and "data/tad-heritages.tsv" files.

In each file, the IDs for each set of data need to be unique. The names of the columns are written in snake_case, so they can be deserialized properly in the data loader.

### Room TSV Formatting

The columns in the room tsv file are:
ID - A unique positive integer identifier for the room.
Name - A ideally short string name for the room.
Description - A description that is printed for the player when they first enter the room.
Exits - A formatted list of exits for the room. The format is the name and id seperated by a colon, name and id pairs are seperated by a comma.
Features - A list of things that can be interacted with in a room. This column can be blank. Not fully implemented on the game side of things.

```
id	name	description	exits	features
1	Old Road	You're standing in the middle of an old worn road on the outskirts of a grand city, which stands before you to the north.	north:2
2	City South Gate	You stand in front of the South Gate into the City of SomeName. Guards stand on either side of the entrance.	north:3, south:1	
```

### Heritage TSV Formatting

The colums in the heritage tsv file are:
ID - A unique positive integer identifier for the heritage.
Name (Singular) - The singular name of the heritage.
Name (Plural) - The plural name of the heritage.
Description - A description that is printed for the player during character creation.
Bonuses - Traits that as associated with that heritage. Currently unimplemented in game. In the future this will probably be changed to features or something like that.


```
id	name_singular	name_plural	description	bonuses
1	Human	Humans	A generic humanoid.
2	Dwarf	Dwarves	A short, stocky humanoid with long, respectable beards.
3	Elf	Elves	A humanoid with pointy ears.
```