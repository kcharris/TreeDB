<script setup>
    import {VLabel, VContainer} from "vuetify/components";
</script>


<template>
    <v-sheet width="70dvh" class="mx-auto mt-10" rounded :elevation="10">
        <v-container>
            <p class="text-h1">Settings</p>
            <v-divider></v-divider>
            <br>
            <v-list lines="one">
                <div class="d-flex">
                    <p class="my-auto text-h4">Dark Mode</p>
                    <v-spacer></v-spacer>
                    <v-switch color="primary" class="my-auto"></v-switch>
                </div>
                <v-divider></v-divider>
            </v-list>
        </v-container>
    </v-sheet>
</template>