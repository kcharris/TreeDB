## Personal Database for Desktop
Currently under development, a release of the application for Windows will be out in a few days. If you want to work on it, follow the Tauri installation guide [here](https://tauri.app/v1/guides/getting-started/prerequisites) and then start the application using commands on this development page [here](https://tauri.app/v1/guides/development/development-cycle).

#### To-do
* Settings Page
* DB Management page
* Tagging System

### Overview
This is a tool to manage user created nested lists of items, that can be kept track of over time, that don't want to be forgotten. A few examples include: movies, books, music, games, art projects, chores around the house, and cooking ideas. Tens to hundreds of things in each category can build up over time and become hard to manage. This tools aims to make their management easier and more efficient.  

General categories can be broken down until items are actionable. An example of this would be a Fun category that breaks down into movies, comics, books, and games. Then, each of these subcategories can be filled with specific items. A specific movie may be actionable because it can be watched. In addition, under a movie can include items such as "look up the music" or "find out who the actors are."  

This tool is best used in conjunction with a calendar, where items can be placed for use.  

### Ease of Use
* Quick Search within every layer by typing in the search bar. Items are filtered on input using subsequences.
* Multi-sort by data headers. Data headers can be given priority in the sort process by clicking between them.
* Date fields use calendar dialog boxes.
* The only required field is Name, and a few items are given a default to make item creation quicker.
* Quickly create and edit items with easy to access dialog boxes.

### Data
This tool generalizes items and their properties for management. Here are the properties of each item.

| | | | | |
| --- | --- | --- | --- | --- |
| Name    | Parent | Priority | Estimated Time | Start Date |
| End Date | Resource | Availability | Completed | Description |

### Tools Used
* Tauri
* Rust
* Vue
* Typescript
* Vuetify
* SQLite
* SeaORM
* VS Code
