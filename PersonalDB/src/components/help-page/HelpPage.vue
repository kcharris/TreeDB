<script setup lang="ts">

</script>


<template>    
    <v-sheet color="teal-lighten-4" class="h-100 mx-auto w-100">
        <v-container>
            <v-toolbar class="w-75 mx-auto" color="blue-grey-lighten-5" density="compact">
                    <p class="ml-5">Help and Info Page</p>
                    <v-spacer/>
            </v-toolbar>
            <v-sheet class="mx-auto w-75 overflow-x-auto" height="85dvh">
                <v-card variant="elevated" class="mx-auto">
                    <v-card-title >Help</v-card-title>
                    <v-card-text>
                        <li><b>Sorting:</b> On the list page items can be sorted from the header bar. Click a header to sort based on the header. It is possible to sort by multiple headers at once.</li>
                        <li><b>Searching:</b> On the list page items can be filtered on input using subsequences. Which means searching with "abc" will match "afgbqc" because "afgbqc" contains the search letters in the same order they appear.</li>
                        <li><b>Filtering:</b> Add a tag from the tag page to an item from the edit popup to find it easier using the Tag Filter.</li>
                        <li><b>Database Selection:</b> Once a database has been created, select it from the select column to make it the active database.</li>
                        <li><b>Default Database:</b> The last selected database will be the database to open on startup.</li>
                        <li><b>Backups:</b> Backups are currently not automatic.</li>
                        <li><b>Moving Items:</b> Moving items is currently not supported. It may be added in a future release.</li>
                        <li><b>List and Tag page:</b> The List page and Tag page are both a part of the same database. Switching databases will switch both List and Tag Pages.</li>
                    </v-card-text>
                </v-card>
                <v-divider></v-divider>

                <v-card variant="elevated" class="mx-auto">
                    <v-card-title >Info</v-card-title>
                    <v-card-text>
                        Hello! Welcome to my personal database application. This application was made to ease the management of a backlog I used to keep in spreadsheets. I'll be happy if you find some use out of it as well.
                    </v-card-text>
                    <v-card-text>
                        <b>Overview from GitHub</b>
                        <br>
                        This is a tool to manage user created nested lists of items, that can be kept track of over time, that don't want to be forgotten. A few examples include:
                        movies, books, music, games, art projects, chores around the house, and cooking ideas. Tens to hundreds of things in each category can build up over time
                        and become hard to manage. This tools aims to make their management easier and more efficient.
                        <br><br>
                        General categories can be broken down until items are actionable. An example of this would be a Fun category that breaks down into movies, comics, books, and games. Then, each of these subcategories can be filled with specific items. A specific movie may be actionable because it can be watched. In addition, under a movie can include items such as "look up the music" or "find out who the actors are."
                        <br><br>
                        This tool is best used in conjunction with a calendar, where items can be placed for use.
                        <br><br>
                        <b>Tools Used</b>
                        <br>
                        This application was built with Tauri, a desktop development framework that uses Rust for the backend and offers flexability with the frontend. For the frontend I used Typescript with Vue.
                        I used a frontend framework Vuetify for styled components. VS Code as the editor. Finally, this app uses an SQLite database managed with the SeaORM library.
                    </v-card-text>
                </v-card>
                <v-divider></v-divider>
                <v-card variant="elevated" class="mx-auto">
                    <v-card-title>Links</v-card-title>
                    <v-card-text>
                        <b>GitHub</b>
                        <br>
                        Here is a link to the project GitHub: <a href="https://github.com/kcharris/PersonalDatabaseDesktop">https://github.com/kcharris/PersonalDatabaseDesktop</a>     
                    </v-card-text>
                </v-card>
            </v-sheet> 
        </v-container>
    </v-sheet>
</template>

<style>
    th{
        text-align: left;
    }
</style>