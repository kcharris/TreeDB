<script setup lang="ts">
import default_background from "../../assets/forest.jpg"
import { VImg } from "vuetify/components";
</script>

<template>
    <v-card>
        <v-img max-height="25dvh"
            aspect-ratio="1"
            :lazy-src="default_background"
            :src="default_background"
            cover
            >
            <template v-slot:placeholder>
                <div class="d-flex align-center justify-center fill-height">
                    <v-progress-circular
                    color="grey-lighten-4"
                    indeterminate
                    ></v-progress-circular>
                </div>
            </template>
            <v-card-title class="text-white text-h1 text-center">Tree DB</v-card-title>
        </v-img>
    </v-card>
</template>