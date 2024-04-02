<script setup>
import {ref} from "vue";
    defineProps({
        name: String
    })
    const dialog = ref(false)
  
</script>
<template>
    <div>
        <v-text-field
        :label="name"
        hint="Use YYYY-MM-DD format"
        prepend-inner-icon="mdi-calendar"
        @click:prepend-inner="dialog = true"
        >
        </v-text-field>
        <v-dialog
            v-model="dialog"
            max-width="500"
            >
            <v-card>
                <v-date-picker></v-date-picker>
            </v-card>
            <template v-slot:actions>
                <v-btn
                class="ms-auto"
                text="Ok"
                @click="dialog = false"
                ></v-btn>
            </template>   
        </v-dialog>
    </div>
</template>
