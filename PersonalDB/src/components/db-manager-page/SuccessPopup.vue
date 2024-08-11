<script setup lang="ts">

    const success_dialog = defineModel<boolean>()
</script>

<template>
    <v-dialog
        v-model="success_dialog"
        max-width="500"
        >
        <v-card >
            <v-card-title>Backup Confirmation</v-card-title>
            <v-card-text>Backup has been successfully created.</v-card-text>
            
            <v-card-actions>
                <v-btn
                    class="justify-end bg-primary"
                    text="Confirm"
                    @click="success_dialog=false"
                ></v-btn>
            </v-card-actions>
        </v-card>
            
    </v-dialog>
</template>