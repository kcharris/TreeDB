# Tree DB for Desktop
[A release version can be found here](https://github.com/kcharris/TreeDB/releases/tag/v1.0.1)

## Overview
TreeDB is a desktop application for managing nested lists of items. This tool aims to help users manage items with ease and efficiency.

## Data Model
This tool generalizes items and their properties for management.

<img src="AppImages/ER%20Diagram.png" name="ER Diagram" width=50%>

## Ease of Use
* Quick Search within every layer by typing in the search bar. Items are filtered on input using subsequences.
* A Tag management system that allows items to be filtered by Tag.
* Multi-sort by data headers. Data headers can be given priority in the sort process by clicking between them.
* Calendar boxes where dates can be selected.
* Quickly create and edit items with easy to access dialog boxes.

## Database Management
* Allows the CRUD management of different databases so users can keep different lists.
* Allows users to backup databases where they can be managed on another page.

## Built With
| Tech         | Purpose                     |
|--------------|-----------------------------|
| Tauri      | Desktop app framework       |
| Rust       | Backend logic               |
| Vue.js     | Frontend UI framework       |
| TypeScript | Strongly typed JS         |
| Vuetify    | Material Design UI components |
| SQLite     | Local database engine       |
| SeaORM     | ORM for Rust + SQLite       |
| VS Code      | Development environment     |

## Application screenshots
<p float="left" align="center">
  <img src="AppImages/MainPageView.jpg" name="MainPage" width="100%">
  <img src="AppImages/EditView.jpg" name="EditPage" width="100%">
  <img src="AppImages/TagPageView.jpg" name="TagPage" width="100%">
  <img src="AppImages/DBPageView.jpg" name="DBPage" width="100%">
</p>
